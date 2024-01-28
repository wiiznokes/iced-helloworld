use iced::{executor, widget::Text, Application, Command, Settings};

fn main() {
    App::run(Settings::default()).unwrap()
}
struct App {}

#[derive(Debug, Clone)]
enum AppMsg {}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("App")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message, Self::Theme, iced::Renderer> {
        Text::new("hello").into()
    }

    
}
