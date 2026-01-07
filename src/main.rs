mod files;
mod state;
mod workspace;

fn main() -> iced::Result {
    iced::application(
        state::State::default,
        state::State::update,
        state::State::view,
    )
    .title("Birdlestein")
    .theme(|_: &state::State| iced::Theme::CatppuccinMocha)
    .run()
}
