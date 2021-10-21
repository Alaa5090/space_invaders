use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color,SetBackgroundColor};
use crossterm::terminal::{Clear,ClearType};
use crossterm::QueueableCommand;
use std ::io::{Stdout,Write};
pub fn render(stdout:&mut Stdout, last_frame :&Frame ,curr_frame: &Frame, force:bool){
unimplemented!()
}
