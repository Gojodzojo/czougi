use super::input::InputState;
use super::options::Options;
use crossterm::Result;
use std::io::Stdout;
use std::time::Duration;

pub mod editor;
pub mod game_picker;
pub mod menu;

pub const SIDEBAR_WIDTH: u16 = 25;
pub trait Mode {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        delta_time: Duration,
        horizontal_margin: u16,
        vertical_margin: u16,
        refresh: bool,
        input_state: &InputState,
        options: &Options,
    ) -> Result<Option<Box<dyn Mode>>>;
}
