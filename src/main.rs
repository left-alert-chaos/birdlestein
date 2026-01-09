use iced::{Font, application};
mod config;
mod files;
mod state;
mod workspace;

use state::State;

//i give you: the app itself
fn main() -> iced::Result {
    //function to define settings and default state
    fn state_definer() -> State {
        let settings = config::Settings::from_file("birdlestein.toml");
        State {
            config: settings,
            ..Default::default()
        }
    }

    //run application and capture result
    application(state_definer, State::update, State::view)
        .title("Birdlestein")
        .subscription(State::event_subscription)
        .theme(|_: &state::State| iced::Theme::CatppuccinMocha)
        .default_font(Font::MONOSPACE)
        .run()
}
