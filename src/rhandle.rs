//! 
//!
//! 

pub struct RHandle{
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    
}

impl RHandle {

    pub fn new(l: i32,t: i32,r: i32,b: i32)->RHandle {
        RHandle{
            left: l,
            top: t,
            right: r,
            bottom: b, 
        }
    }

    pub fn width(&self)->i32 {
        (self.right-self.left)
    }

    pub fn height(&self)->i32 {
        (self.bottom-self.top)
    }

    pub fn is_empty(&self)->bool {
        (self.right==self.left) && (self.bottom==self.top)
    }

    pub fn empty(&mut self){
        self.left = 0;
        self.right = 0;
        self.top = 0;
        self.bottom = 0;
    }

    pub fn pt_in_rect(&self,x: i32,y: i32)->bool {
        (x>=self.left)&&(x<=self.right)&&(y>=self.top)&&(y<=self.bottom)
    }

    pub fn normalize(&mut self) {
        if self.left>self.right {
            let dum = self.left;
            self.left = self.right;
            self.right = dum;
        }
        if self.top>self.bottom {
            let dum = self.top;
            self.top = self.bottom;
            self.bottom = dum;
        }
    }

}
