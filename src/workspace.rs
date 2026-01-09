//workspace.rs - render the UI
use crate::state::{Message, PopupType, State, Tab};
use iced::{
    Font,
    Element, Fill, highlighter,
    widget::{Stack, Text, button, center, column, opaque, scrollable, text_editor},
};
use iced_aw::{TabLabel, card, menu, menu_bar, menu_items, widget::tab_bar::TabBar};
use std::fs;

//This function is single-handedly responsible for the vast majority of the GUI
pub fn render_workspace(state: &State) -> Element<'_, Message> {
    //message if no tabs are open
    if state.tabs.len() == 0 {
        return center(button("Open file").on_press(Message::OpenFile)).into();
    }

    let current_tab: &Tab = &state.tabs[state.tab_id];

    //setup tab bar
    let mut bar = TabBar::new(Message::TabChanged).on_close(Message::CloseTab);
    for (id, tab) in state.tabs.clone().into_iter().enumerate() {
        bar = bar.push(id, TabLabel::from(tab.title));
    }
    bar = bar.set_active_tab(&state.tab_id);

    //draw main area: the 2d area with tabs, text, etc
    let main_area = column![
        //shenanigans to get the menu bar working
        menu_bar!(
            (
                button("File").on_press(Message::MenuOpened),
                menu!(
                    (button("Open...").on_press(Message::OpenFile)),
                    (button("New...").on_press(Message::NewFile)),
                    (button("Save").on_press(Message::Save)),
                    (button("Close").on_press(Message::CloseCurrent)),
                )
                .width(200)
            ),
            (
                button("Help").on_press(Message::MenuOpened),
                menu!((button("License").on_press(Message::ShowLicense)),).width(250)
            )
        )
        .width(100),
        bar,
        //file path
        match &state.tabs[state.tab_id].file_path {
            Some(path) => {
                path.as_str()
            }
            None => "No file path",
        },
        //configure editor
        text_editor(&current_tab.content)
            .highlight("rs", highlighter::Theme::SolarizedDark)
            .on_action(Message::TextEdited)
            .font(Font::MONOSPACE),
    ];

    //depth is a stack - it lets widgets be on top of each other. Main rendering should happen in main_area
    let mut depth: Stack<Message> = Stack::new();
    depth = depth.push(main_area);

    if state.popup == PopupType::License {
        depth = depth.push(opaque(scrollable(
            card(
                Text::new("Birdlestein License"),
                Text::new(fs::read_to_string("LICENSE").unwrap()),
            )
            .on_close(Message::HidePopup)
            .height(Fill),
        )));
    }

    depth.into()
}