use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{color,setBackgroundcolor};
use crossterm::terminal::{clear,clearType};
use crossterm::Queueablecommand;
use std ::io::{stdout,write};
pub fn render(stdout:&mut stdout, last_frame :&frame ,curr_frame: &Frame, force:bool){
unimplemented!()
}
