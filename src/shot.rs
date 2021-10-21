use crate::frame::{Drawable,Frame};
use std::time::Duration;

pub struct Shot{
pub x :usize,
pub y :usize,
pub exploding : bool,

}
impl Shot {
    pub fn new (x:usize,y:usize)->Self{
        unimplemented!()
    }


    pub fn update(&mut self,delta:Duration){
        unimplemented!()
    }

    pub fn explode(&mut self){
        unimplemented!()
    }
    pub fn dead(&self)->bool{
        unimplemented!()
    }

}


impl Drawable for Shot {
  fn draw(&self, frame: &mut Frame){
        unimplemented!()
    }
    
}