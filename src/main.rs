use iced::{executor, widget::Text, Application, Command, Element, Settings};

fn main() {
    let settings = Settings::default();

    App::run(settings).unwrap()
}
struct App {}

#[derive(Debug, Clone)]
enum AppMsg {}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("App")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("hello").into()
    }
}
