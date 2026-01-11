//workspace.rs - render the UI
use crate::{state::{Message, MenuMessage, PopupType, State, Tab, FileDisplay, FileDisplayType}, config::Project};
use iced::{
    Element, Font, Length, highlighter,
    widget::{Stack, Text, button, center, column, opaque, scrollable, text_editor, row},
};
use iced_aw::{TabLabel, card, menu, menu_bar, menu::Item, menu_items, widget::tab_bar::TabBar};
use std::{fs};

//This function is single-handedly responsible for the vast majority of the GUI
pub fn render_workspace(state: &State) -> Element<'_, Message> {
    //shenanigans to get the menu bar working
    let menus = menu_bar!(
        (
            button("File").on_press(Message::MenuOpened),
            menu!(
                (button("Open...").on_press(Message::OpenFile)),
                (button("New...").on_press(Message::NewFile)),
                (button("Save").on_press(Message::Save)),
                (button("Close").on_press(Message::CloseCurrent)),
            )
            .width(Length::Shrink)
        ),
        (
            button("Project").on_press(Message::MenuOpened),
            menu!(
                (button("Open...").on_press(Message::MenuOpened), menu!(/*{
                    let mut items = Vec::new();

                    let projects = state.config.projects.clone().into_iter();

                    for (name, project) in projects {
                        items.push((button(&name[..]).on_press(Message::MenuMessage(MenuMessage::OpenProject(name)))));
                    }
                    menu_items!(items)
                }*/(button("hi").on_press(Message::MenuMessage(MenuMessage::OpenProject(String::from("myProject")))))).width(Length::Shrink))
            ).width(Length::Shrink)
        ),
        (
            button("Help").on_press(Message::MenuOpened),
            menu!((button("License").on_press(Message::ShowLicense))).width(Length::Shrink)
        )
    )
    .width(Length::Fill);

    //return simplified view if no tabs
    if state.tabs.len() == 0 {
        return column![
            menus,
            center(button("Open file").on_press(Message::OpenFile)),
        ]
        .into();
    }

    let current_tab: &Tab = &state.tabs[state.tab_id];

    //setup tab bar
    let mut tabs = TabBar::new(Message::TabChanged).on_close(Message::CloseTab);
    for (id, tab) in state.tabs.clone().into_iter().enumerate() {
        tabs = tabs.push(id, TabLabel::from(tab.title));
    }
    tabs = tabs.set_active_tab(&state.tab_id);

    //work area is everything below the menu bar
    let work_area = row![
        //side panel
        draw_side_panel(state),

        //tabbed area
        column![
            tabs,
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
        ],
    ];

    //draw main layer: the 2d area with bar, tabs, text, etc
    let main_layer = column![
        menus,
        work_area,
    ];

    //depth is a stack - it lets widgets be on top of each other. Main rendering should happen in main_area
    let mut depth: Stack<Message> = Stack::new();
    depth = depth.push(main_layer);

    //draw popup if popup is showing
    if let Some(ptype) = &state.popup {
        match ptype {
            PopupType::License => {
                depth = depth.push(opaque(scrollable(
                    card(
                        Text::new("Birdlestein License"),
                        Text::new(fs::read_to_string("LICENSE").unwrap()),
                    )
                    .on_close(Message::HidePopup)
                    .height(Length::Fill),
                )));
            }
        }
    }

    depth.into()
}

pub fn draw_side_panel(state: &State) -> Element<'_, Message> {
    let mut area = column![];
    for display in &state.file_displays {
        match display.file_type {
            FileDisplayType::File => {
                area = area.push(button(display.name.as_str()).on_press(Message::OpenSpecificFile(display.name.clone())));
            }
            FileDisplayType::Directory => {

            }
        }
    }
    area.into()
}