use iced::{executor, widget::Text, Application, Command, Settings, window::{Icon, self}};

fn main() {

    // https://stackoverflow.com/questions/30291757/attaching-an-icon-resource-to-a-rust-application
    let bytes = include_bytes!("./../resource/app_icon/app_icon150.ico");

    // todo: export Imageformat from Iced
    let icon = window::icon::from_file_data(bytes, None).unwrap();   

    let settings = Settings {



        window: iced::window::Settings { 
             icon: Some(icon), 
             .. iced::window::Settings::default()
            },
        .. Settings::default()
    };

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

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("App")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Text::new("hello").into()
    }
}
