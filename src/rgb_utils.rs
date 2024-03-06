//! 
//!
//! 

const RGB_R_MASK: u32 = 0xFF << 24;
const RGB_G_MASK: u32 = 0xFF << 16;
const RGB_B_MASK: u32 = 0xFF << 8;
const RGB_A_MASK: u32 = 0xFF;

pub fn rgba(red: u8,green: u8,blue: u8,alpha: u8)->u32 {
    let b = (blue as u32) << 8;
    let g = (green as u32) << 16;
    let r = (red as u32) << 24;
    let a = alpha as u32;
    (r | g | b | a)
    
}

pub fn get_rgba_a(rgba: u32)->u8 {
    (rgba & RGB_A_MASK) as u8
}

pub fn get_rgba_r(rgba: u32)->u8 {
    ((rgba & RGB_R_MASK) >> 24) as u8
}

pub fn get_rgba_g(rgba: u32)->u8 {
    ((rgba & RGB_G_MASK) >> 16) as u8
}

pub fn get_rgba_b(rgba: u32)->u8 {
    ((rgba & RGB_B_MASK) >> 8) as u8
}

pub fn get_rgba(rgba: u32)->(u8,u8,u8,u8) {
    let r = ((rgba & RGB_R_MASK) >> 24) as u8;
    let g = ((rgba & RGB_G_MASK) >> 16) as u8;
    let b = ((rgba & RGB_B_MASK) >> 8) as u8;
    let a = (rgba & RGB_A_MASK) as u8;
    (r,g,b,a)
}
