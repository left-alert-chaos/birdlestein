//state.rs - manage application state
//holds state struct, tab struct, and message enum
use crate::{files, workspace};
use iced::{
    Element,
    widget::text_editor::{Action, Content},
};
use std::{fs, path::Path, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    TextEdited(Action),
    CloseTab(usize),
    TabChanged(usize),
    Save,
    NewFile,
    //MenuMessage(MenuMessage),
    MenuOpened,
    CloseCurrent,
    ShowLicense,
    HidePopup,
    MenuMessage(MenuMessage),
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum PopupType {
    #[default]
    License,
}

//messages specifically from dropdown menus
#[derive(Debug, Clone)]
pub enum MenuMessage {
    OpenProject(String),
}

//hold all info about an editor tab
#[derive(Default, Clone)]
pub struct Tab {
    pub(crate) title: String,
    pub(crate) file_path: Option<String>,
    pub(crate) content: Content,
}

impl Tab {
    pub fn open_file() -> Self {
        let file: PathBuf = match files::pick_file() {
            Some(path) => path,
            None => return fileless_tab(),
        };

        let file_text: String = fs::read_to_string(file.clone()).unwrap_or(String::from(""));
        //https://stackoverflow.com/questions/37388107/how-to-convert-the-pathbuf-to-string
        let file_path: String = file
            .clone()
            .into_os_string()
            .into_string()
            .expect("FILE PATH NOT CONVERTABLE TO STRING");
        let title: String = get_title(&file_path);

        Tab {
            title,
            file_path: Some(file_path),
            content: Content::with_text(&file_text),
        }
    }
}

#[derive(Default)]
pub struct State {
    pub(crate) config: crate::config::Settings,
    pub(crate) tabs: Vec<Tab>,
    pub(crate) tab_id: usize,
    pub(crate) popup: Option<PopupType>,
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenFile => {
                self.tabs.push(Tab::open_file());
                self.tab_id = self.tabs.len() - 1;
            }
            Message::TextEdited(action) => {
                self.tabs[self.tab_id].content.perform(action);
            }
            Message::TabChanged(id) => {
                println!("Switched to tab {id}");
                self.tab_id = id;
            }
            Message::CloseTab(id) => {
                self.close_tab(id);
            }
            Message::MenuOpened => {
                println!("A menu was opened.");
            }
            Message::Save => {
                self.save();
            }
            Message::CloseCurrent => {
                self.save();
                self.close_tab(self.tab_id);
            }
            Message::ShowLicense => {
                self.popup = Some(PopupType::License);
            }
            Message::HidePopup => {
                self.popup = None;
            }
            Message::NewFile => {
                self.new_file(true);
            }
            _ => {
                println!("Unknown message: {message:?}")
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        workspace::render_workspace(self)
    }

    pub fn close_tab(self: &mut State, id: usize) {
        println!("Removing tab {id}");

        //safety
        if self.tab_id == id && self.tab_id > 0 {
            self.tab_id -= 1
        }

        self.tabs.remove(id);
    }

    //handle file saves
    pub fn save(self: &mut State) {
        //get current tab and see if it has a file path
        let tab: &mut Tab = &mut self.tabs[self.tab_id];
        if let Some(path) = &tab.file_path {
            files::write_file(path, tab.content.text())
        } else {
            //if no file path, ask for one. If not given, don't save.
            self.new_file(false);
        }
    }

    //ask to choose a file to save as. new_tab controls whether to use current tab or to make new tab.
    pub fn new_file(self: &mut State, new_tab: bool) {
        //if new tab, create new tab, switch to it, and go from there
        if new_tab {
            self.tabs.push(Tab {
                title: String::from("New file"),
                file_path: None,
                content: Content::new(),
            });
            self.tab_id = self.tabs.len() - 1;
        }

        let tab: &mut Tab = &mut self.tabs[self.tab_id];
        let path = files::save_file();
        match path {
            Some(file_path) => {
                let string_path = file_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .expect("FILE PATH NOT CONVERTABLE TO STRING");
                tab.file_path = Some(string_path.clone());
                tab.title = get_title(&string_path);
                files::write_file(&string_path, tab.content.text());
            }
            None => return,
        }
    }
}

pub fn fileless_tab() -> Tab {
    Tab {
        title: String::from("No file"),
        file_path: None,
        content: Content::new(),
    }
}

pub fn get_title(file_path: &String) -> String {
    String::from(Path::new(&file_path).file_name().unwrap().to_str().unwrap())
}
