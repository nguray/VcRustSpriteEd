//! 
//!
//!
//! 
//!
//!

#[derive(PartialEq, Copy, Clone)]
pub struct SelectRect{
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub mode: i32,
    pub sel_corner: i32,
    pub mouse_start_x: f64,
    pub mouse_start_y: f64,
    pub sav_left: i32,
    pub sav_top: i32,
    pub sav_right: i32,
    pub sav_bottom: i32,

}


impl SelectRect {

    pub fn new(l: i32,t: i32,r: i32,b: i32)->SelectRect {
        SelectRect{
            left: l,
            top: t,
            right: r,
            bottom: b,
            mode: 0,
            sel_corner: -1,
            mouse_start_x: 0.0,
            mouse_start_y: 0.0,
            sav_left: 0,
            sav_top: 0,
            sav_right: 0,
            sav_bottom: 0,
               }
    }
    
    pub fn backup_position(&mut self){
        self.sav_left = self.left;
        self.sav_top = self.top;
        self.sav_right = self.right;
        self.sav_bottom = self.bottom;
    }

    pub fn restore_position(&mut self){
        self.left = self.sav_left;
        self.top = self.sav_top;
        self.right = self.sav_right;
        self.bottom = self.sav_bottom;
    }

    pub fn get_corner(&self,n:i32)->(i32,i32) {
        if n==0 {
            (self.left,self.top)
        }else if n==1 {
            (self.right,self.top)
        }else if n==2 {
            (self.right,self.bottom)
        }else if n==3 {
            (self.left,self.bottom)
        }else{
            (0,0)
        }
    }

    pub fn set_corner(&mut self,n: i32,x: i32,y: i32) {
        if n==0 {
            self.left = x;
            self.top = y;
        }else if n==1 {
            self.right = x;
            self.top = y;
        }else if n==2 {
            self.right = x;
            self.bottom = y;
        }else if n==3 {
            self.left = x;
            self.bottom = y;
        }
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

    pub fn empty(&mut self){
        self.left = 0;
        self.right = 0;
        self.top = 0;
        self.bottom = 0;
    }
    
    pub fn is_empty(&self)->bool {
        (self.right==self.left) || (self.bottom==self.top)
    }

    pub fn init(&mut self) {
        self.empty();
        self.mode = 0;
        self.mouse_start_x = 0.0;
        self.mouse_start_y = 0.0;
    }

    pub fn width(&self)->i32 {
        self.right-self.left+1
    }

    pub fn height(&self)->i32 {
        self.bottom-self.top+1
    }

    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.left  += dx;
        self.right += dx;
        self.top   += dy;
        self.bottom+= dy;
    }
}
