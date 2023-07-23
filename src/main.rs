use iced::{Application, executor, Command, widget::Text, Settings};

fn main() {
    App::run(Settings::default()).unwrap()
}




struct App {

}

#[derive(Debug)]
enum AppMsg {
    
}


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

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
       
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Text::new("hello")
        .into()
    }
}
