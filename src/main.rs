use app::App;
mod app;
mod config;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<App>(true, ())
}
