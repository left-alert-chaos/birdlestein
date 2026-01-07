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
    //MenuMessage(MenuMessage),
    MenuOpened,
    CloseCurrent,
}

//messages specifically from dropdown menus
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MenuMessage {
    OpenFile,
    SaveFile,
    PrintHello,
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
        let title: String =
            String::from(Path::new(&file_path).file_name().unwrap().to_str().unwrap());

        Tab {
            title,
            file_path: Some(file_path),
            content: Content::with_text(&file_text),
        }
    }
}

#[derive(Default)]
pub struct State {
    pub(crate) tabs: Vec<Tab>,
    pub(crate) tab_id: usize,
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenFile => {
                self.tabs.push(Tab::open_file());
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

    pub fn save(self: &State) {
        let tab: &Tab = &self.tabs[self.tab_id];
        if let Some(path) = &tab.file_path {
            files::write_file(path, tab.content.text())
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
