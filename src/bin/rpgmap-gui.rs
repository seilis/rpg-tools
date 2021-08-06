//! GUI for rpgmap
//!
//! This is a GUI for the rpgmap tool; it will eventually allow the user to automatically generate
//! maps and/or edit them by hand.
//!
//! # How to run
//! ```
//! rpgmap-gui
//! ```
//!
//! # How this code is organized
//!
//! This crate contains just the top-level declarations for the [iced](https://github.com/hecrj/iced) library; most of the actual
//! implementation is in `rpgtools/gui` and the actual map logic and generation is in `rpgtools/map`.
//!
//! Note that this app borrows very heavily on the ideas and examples presented in iced's [Game of
//! Life](https://github.com/hecrj/iced/blob/master/examples/game_of_life/src/main.rs) example.
use iced::{
    Application,
    Clipboard,
    Column,
    Command,
    Container,
    Element,
    Length,
    Settings,
    Subscription,
    executor,
};

use rpgtools::gui::map::{Controls, Message};

pub fn main() -> iced::Result {
    RpgMapGui::run(Settings::default())
}

/// Main state for the program
struct RpgMapGui {
    controls: Controls,
}

/// Iced Application implementation for the RPG Map GUI
///
/// This implements the Application state, according to the iced trait of the same name.
impl Application for RpgMapGui {
    /// I don't even know what this is yet... but the examples say to use the default.
    type Executor = executor::Default;
    /// Internal messages that the Application produces/consumes
    type Message = Message;
    /// Help text says this is data needed to init the application.
    type Flags = ();

    /// Make a new application; we don't need any initial state
    fn new(_flags: ()) -> (RpgMapGui, Command<Message>) {
        (
            RpgMapGui {
                controls: Controls::default(),
            },
            Command::none(),
        )
    }

    /// Title of the window for the app
    fn title(&self) -> String {
        String::from("RPG Map")
    }

    /// Handler for updating the state from a message
    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            _ => {}
        }
        Command::none()
    }

    /// Not sure what this does yet
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    /// Method to draw the application
    fn view(&mut self) -> Element<Message> {

        let controls = self.controls.view();

        let content = Column::new()
            .spacing(20)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
