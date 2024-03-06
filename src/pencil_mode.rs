//use gio::prelude::*;
use gtk::prelude::*;

use crate::edit_area::{EditAreaDatas,UndoMode};
use crate::rgb_utils::{*};
use gdk_pixbuf::*;


pub trait PencilMode {
    
    fn draw_pencil_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context);
    fn button_press_event_pencil_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn button_release_event_pencil_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn motion_notify_event_pencil_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion);
    fn get_pixel_color(&self, sprite: &Pixbuf, x: i32, y: i32)->(u32,bool);

}

impl PencilMode for EditAreaDatas{

    fn draw_pencil_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context) {
        let w = _widget.get_allocation().width - 1;
        let h = _widget.get_allocation().height - 1;

        //-- Compute Cells size
        self.compute_cell_size(w, h);

        cr.set_source_rgba( 1.0, 1.0, 1.0, 1.0);
        cr.paint();

        //--
        cr.translate(self.origin_x as f64, self.origin_y as f64);

        cr.set_source_rgba( 1.0, 252.0/255.0, 242.0/255.0, 1.0);
        cr.rectangle( 1.0, 1.0, (self.nb_pixs_columns*self.cell_size) as f64, (self.nb_pixs_rows*self.cell_size) as f64);
        cr.fill();

        // Examples are in 1.0 x 1.0 coordinate space
        //cr.scale(120.0, 120.0);

        cr.set_source_rgba( 0.0, 0.0, 0.0, 1.0);
        self.draw_frame( &_widget, &cr);

        self.draw_pixels( &cr);

    }

    fn get_pixel_color(&self, sprite: &Pixbuf, x: i32, y: i32)->(u32,bool)
    {
        unsafe {

            let width  = sprite.get_width();  
            let height = sprite.get_height(); 

            if x>=0 && x<width && y>=0 && y<height {
     
                let row_stride = self.sprite.get_rowstride();
                let n_channels = self.sprite.get_n_channels();
                let pixels = self.sprite.get_pixels();

                let ip = (y * row_stride + x * n_channels) as usize;
                let red     = (pixels[ip] as u8);
                let green   = (pixels[ip+1] as u8);
                let blue    = (pixels[ip+2] as u8);
                let alpha   = (pixels[ip+3] as u8);

                (rgba(red, green, blue, alpha),true)

            }else{
                (0,false)
            }
        }

    }

    fn button_press_event_pencil_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {

        //let widget1 = self.get_instance();
        //let b: bool = state.contains(CONTROL_MASK);

        if let Some((x, y)) = event.get_coords(){

            let state: gdk::ModifierType = event.get_state();

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);
        

            let draw_color = if event.get_button()==1 {
                self.foreground_color
            }else if event.get_button()==3 {
                self.background_color
            }else{
                self.foreground_color       
            };

            self.undo_mode = UndoMode::PENCIL;

            if state.contains(gdk::ModifierType::CONTROL_MASK){
                self.backup_sprite();
                let (px,py) = self.mouse_to_pixel( tmx, tmy);
                if self.last_pixel_x>=0 || self.last_pixel_y>=0 {
                    self.draw_line( &self.sprite, self.last_pixel_x, self.last_pixel_y, px, py, draw_color);
                    _widget.queue_draw();
                    _widget.emit("edit-changed",&[]);
                }
            }else if state.contains(gdk::ModifierType::SHIFT_MASK){
                let (px,py) = self.mouse_to_pixel( tmx, tmy);
                let (pick_color, f_ok) = self.get_pixel_color( &self.sprite,px , py);
                if event.get_button()==1{
                    self.foreground_color = pick_color;
                    _widget.emit("pick-foreground-color",&[&pick_color]);
                }else{
                    self.background_color = pick_color;
                    _widget.emit("pick-background-color",&[&pick_color]);
                }
            }else{
                let (px,py) = self.mouse_to_pixel( tmx, tmy);
                if self.in_edit_area(px,py){
                    self.backup_sprite();
                    let (r, g, b, a) = get_rgba(draw_color);
                    self.sprite.put_pixel(px as u32, py as u32, r, g, b, a);
                    //println!("Press x={} y={}", px, py);
                    _widget.queue_draw();
                }
                let mx: u32 = x as u32;
                let my: u32 = y as u32;
                //_widget.emit("left-clicked", &[&mx,&my]);
                _widget.emit("edit-changed",&[]);
                //_widget.emit("edit-modify",&[&tmx]);

            }
        }
    }

    fn button_release_event_pencil_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {
        //--
        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let (px,py) = self.mouse_to_pixel( tmx, tmy);
            self.last_pixel_x = px;
            self.last_pixel_y = py;
            //println!("Release px={} py={}", px, py);
            _widget.emit("edit-changed",&[]);

        }
    }

    fn motion_notify_event_pencil_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion) {

        //---

        if let Some((x, y)) = event.get_coords() {

            let state: gdk::ModifierType = event.get_state();

            let draw_color = if state.contains(gdk::ModifierType::BUTTON1_MASK) {
                self.foreground_color
            }else if state.contains(gdk::ModifierType::BUTTON3_MASK) {
                self.background_color
            }else{
                1024    
            };

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            //let widget1 = self.get_instance();
            //let f:bool = state.contains(gdk::ModifierType::BUTTON1_MASK);
            //if state.contains(gdk::ModifierType::BUTTON1_MASK) && state.contains(gdk::ModifierType::CONTROL_MASK) {
            if draw_color!=1024 {
                if state.contains(gdk::ModifierType::CONTROL_MASK){
                    //--
                    let (px,py) = self.mouse_to_pixel( tmx, tmy);
                    if self.in_edit_area(px,py){
                        self.restore_sprite();
                        if self.last_pixel_x>=0 || self.last_pixel_y>=0 {
                            self.draw_line( &self.sprite, self.last_pixel_x, self.last_pixel_y, px, py, draw_color);
                            _widget.queue_draw();
                            _widget.emit("edit-changed",&[]);
                        }
                    }
                }else{
                    //--
                    let (px,py) = self.mouse_to_pixel( tmx, tmy);
                    if self.in_edit_area(px,py){
                        let (r,g,b,a) = get_rgba(draw_color);            
                        self.sprite.put_pixel(px as u32, py as u32, r, g, b, a);
                        _widget.queue_draw();
                        _widget.emit("edit-changed",&[]);
                        //println!("Pencil Mode : Button Motion x={} y={}", x, y);
                    }
                }
            }

        }
       
    }


}