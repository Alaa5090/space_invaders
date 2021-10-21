use crate::frame::{Drawable,Frame};
use crate::{NUM_ROWS,NUM_COLS};
pub struct Invader{
    pub x:usize,
    pub y:usize,
}
pub struct Invaders{
    pub army:Vec<Invader>,
    direction:i32,
}

impl Invaders{
    pub fn new()->Self{
        unimplemented!()
    }

    pub fn update(&mut self,delta:std::time::Duration)->bool{
        unimplemented!()
    }

    pub fn all_killed(&self)->bool{
        unimplemented!()
    }

    pub fn reached_bottom(&self)->bool{
        unimplemented!()
    }

    pub fn kill_invader_at(&mut self, x:usize,y:usize)->bool{
        unimplemented!()
    }
}




impl Drawable for Invaders{
    fn draw(&self,frame: &mut Frame){
        unimplemented!()
    }
}
