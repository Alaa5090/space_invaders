use crate::frame::{Drawable,Frame};
use crate::invaders::Invaders;
use crate::shot::Shot;
use crate::{NUM_COLS,NUM_ROWS};
use std::time::Duration;


pub struct Player{
    x:usize,
    y:usize,
    shots :Vec<Shot>,
}

impl Player {
   
    

       
    

    pub fn update(&mut self,delta:Duration)->bool{
        unimplemented!()
    }
    pub fn move_left(&mut self){
        unimplemented!()
    }
    pub fn move_right(&mut self){
        unimplemented!()
    }
    pub fn shoot (&mut self)-> bool{
        unimplemented!()
    }
    pub fn detect_hits(&mut self,invaders:&mut Invaders)->bool{
        unimplemented!()
    }
}


impl Drawable for Player {
     
        fn draw(frame: &mut Frame){
            unimplemented!()
        }
    }
    
