use std::time::Duration;

use cosmic::app::{Command, Core};
use cosmic::iced::time;
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced_style::application;
use cosmic::iced_widget::row;
use cosmic::widget::{self};
use cosmic::{Application, Element, Theme};
use sysinfo::System;

use crate::config::Config;

#[derive(Default)]
pub struct App {
    core: Core,
    popup: Option<Id>,
    config: Config,
    memory: Memory,
}

#[derive(Default)]
struct Memory {
    total: u64,
    used: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    UpdateMemory,
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "another.lusitano.AppletMemory";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        let seconds = self.config.refresh_time;

        time::every(Duration::from_secs(seconds)).map(|_| Message::UpdateMemory)
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let s = System::new_all();

        let memory = Memory {
            total: s.total_memory(),
            used: s.used_memory(),
        };

        let app = App {
            core,
            memory,
            ..Default::default()
        };

        (app, Command::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<Self::Message> {
        let memory = &self.memory;

        let memory_percentage = (memory.used as f64 / memory.total as f64) * 100.0;

        let percentage = format!("{}%", memory_percentage as u8);

        row![
            self.core.applet.text(percentage),
            self.core
                .applet
                .icon_button("firmware-manager-symbolic")
                .on_press(Message::TogglePopup)
        ]
        .align_items(cosmic::iced::Alignment::Center)
        .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        self.core.applet.popup_container(widget::row()).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::UpdateMemory => {
                let s = System::new_all();

                let memory = Memory {
                    total: s.total_memory(),
                    used: s.used_memory(),
                };

                self.memory = memory;
            }
        }
        Command::none()
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}
