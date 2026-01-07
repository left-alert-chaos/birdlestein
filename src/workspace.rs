//workspace.rs - render the UI
use crate::state::{Message, State, Tab};
use iced::{
    Element, highlighter,
    widget::{button, center, column, text_editor},
};
use iced_aw::{TabLabel, menu, menu_bar, menu_items, widget::tab_bar::TabBar};

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
    column![
        //shenanigans to get the menu bar working
        menu_bar!(
            (
                button("File").on_press(Message::MenuOpened),
                menu!(
                    (button("Open...").on_press(Message::OpenFile)),
                    (button("Save").on_press(Message::Save)),
                    (button("Close").on_press(Message::CloseCurrent)),
                )
                .width(200)
            ),
            (
                button("Help").on_press(Message::MenuOpened),
                menu!((button("hi")),)
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
            .on_action(Message::TextEdited),
    ]
    .into()
}
