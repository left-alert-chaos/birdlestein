use iced::{application, Font};
mod files;
mod state;
mod workspace;
mod config;

//i give you: the app itself
fn main() -> iced::Result {
    //function to define settings and default state
    fn state_definer() -> state::State {
        let settings = config::Settings::from_file("birdlestein.toml");
        state::State {
            config: settings,
            ..Default::default()
        }
    }

    //run application and capture result
    application(
        state_definer,
        state::State::update,
        state::State::view,
    )
    .title("Birdlestein")
    .theme(|_: &state::State| iced::Theme::CatppuccinMocha)
    .default_font(Font::MONOSPACE)
    .run()
}
