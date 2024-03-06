
//use gio::prelude::*;
use gtk::prelude::*;
use gdk_pixbuf::*;

use crate::edit_area::{EditAreaDatas,UndoMode};
use crate::rgb_utils::{*};
use crate::select_mode::SelectMode;

pub trait RectangleMode {
    
    fn draw_rectangle(&self, sprite: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32, c: u32);
    fn draw_rectangle_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context);
    fn button_press_event_rectangle_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn button_release_event_rectangle_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn motion_notify_event_rectangle_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion);
    fn init_rectangle_mode(&mut self);
    fn fill_rectangle(&self, sprite: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32, c: u32);

}


impl RectangleMode for EditAreaDatas{

    fn draw_rectangle(&self, sprite: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32, c: u32)
    {

        let (startX,endX) = if x1>x0 {
                                (x0,x1)
                            }else{
                                (x1,x0)
                            };

        let (startY,endY) = if y1>y0 {
                                (y0,y1)
                            }else{
                                (y1,y0)
                            };

        if (endX!=startX) || (endY!=startY) {
            //-- Tracer le rectangle
            let (r,g,b,a) = get_rgba(c);            
            for x in startX..=endX {
                sprite.put_pixel(x as u32, startY as u32, r, g, b, a);
                sprite.put_pixel(x as u32, endY as u32, r, g, b, a);
            }
            let y1 = startY+1;
            let y2 = endY-1;
            for y in y1..=y2 {
                sprite.put_pixel(startX as u32, y as u32, r, g, b, a);
                sprite.put_pixel(endX as u32, y as u32, r, g, b, a);
            }
            //self.draw_line( &sprite, startX, startY, endX, startY, c);
            //self.draw_line( &sprite, endX, startY, endX, endY, c);
            //self.draw_line( &sprite, endX, endY, startX, endY, c);
            //self.draw_line( &sprite, startX, endY, startX, startY, c);
        }
    
    }

    fn fill_rectangle(&self, sprite: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32, c: u32)
    {

        let (startX,endX) = if x1>x0 {
                                (x0,x1)
                            }else{
                                (x1,x0)
                            };

        let (startY,endY) = if y1>y0 {
                                (y0,y1)
                            }else{
                                (y1,y0)
                            };

        if (endX!=startX) || (endY!=startY) {
            //-- Tracer le rectangle rempli
            let (r,g,b,a) = get_rgba(c);            
            for y in startY..=endY {
                for x in startX..=endX{
                    sprite.put_pixel(x as u32, y as u32, r, g, b, a);
                }
            }
        }
    
    }

    fn init_rectangle_mode(&mut self) {
        self.select_rect.init();
    }

    fn draw_rectangle_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context) {
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

        self.draw_select_rect(&cr);

    }

    fn button_press_event_rectangle_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {

        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let state: gdk::ModifierType = event.get_state();

            //let widget1 = self.get_instance();
            //let b: bool = state.contains(CONTROL_MASK);
            let draw_color  =   if event.get_button()==1 {
                                self.foreground_color
                            }else if event.get_button()==3 {
                                self.background_color
                            }else{
                                self.foreground_color       
                            };
            let (px,py) = self.mouse_to_pixel( tmx, tmy);
            if self.in_edit_area(px,py){

                if self.select_rect.mode==0 {
                    //--
                    self.backup_sprite();
                    self.undo_mode = UndoMode::RECTANGLE;
                    self.select_rect.set_corner( 0, px, py);
                    self.select_rect.set_corner( 2, px, py);
                    _widget.queue_draw();
                    _widget.emit("edit-changed",&[]);

                }else if self.select_rect.mode==1 {
                    if self.in_select_rect( tmx, tmy)==true {
                        let id_handle = self.hit_handle(tmx,tmy);
                        if id_handle!=-1 {
                            //-- Start Mode Handle
                            self.select_rect.sel_corner = id_handle;

                        }else{
                            //-- Start Move Select Rect
                            self.select_rect.mouse_start_x = tmx;
                            self.select_rect.mouse_start_y = tmy;
                            self.select_rect.backup_position();

                        }

                    }else{
                        //self.backup_sprite();
                        self.select_rect.mode = 0;
                        self.select_rect.set_corner( 0, px, py);
                        self.select_rect.set_corner( 2, px, py);
                        self.select_rect.sel_corner = -1;
                        _widget.queue_draw();
                        _widget.emit("edit-changed",&[]);
                           
                    }

                }

                let mx: u32 = x as u32;
                let my: u32 = y as u32;
                _widget.emit("left-clicked", &[&mx,&my]);
                _widget.queue_draw();
                _widget.emit("edit-changed",&[]);
            }
        }
                
    }

    fn button_release_event_rectangle_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {
        //--

        if self.select_rect.mode==0 {
            let (x1,y1) = self.select_rect.get_corner(0);
            let (x2,y2) = self.select_rect.get_corner(2);
            if x1!=x2 && y1!=y2 {
                self.select_rect.normalize();
                self.select_rect.mode = 1;
            }else{
                self.select_rect.init();
            }
            _widget.queue_draw();
            _widget.emit("edit-changed",&[]);
        }
        //--
        self.select_rect.sel_corner = -1;

    }

    fn motion_notify_event_rectangle_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion) {

        //---
        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let state: gdk::ModifierType = event.get_state();

            let draw_color = if state.contains(gdk::ModifierType::BUTTON1_MASK) {
                self.foreground_color
            }else if state.contains(gdk::ModifierType::BUTTON3_MASK) {
                self.background_color
            }else{
                0       
            };

            //let widget1 = self.get_instance();
            //let f:bool = state.contains(gdk::ModifierType::BUTTON1_MASK);
            //if state.contains(gdk::ModifierType::BUTTON1_MASK) && state.contains(gdk::ModifierType::CONTROL_MASK) {
            if draw_color!=0 {

                //--
                let (px,py) = self.mouse_to_pixel( tmx, tmy);
                if self.in_edit_area(px,py){

                    if self.select_rect.mode==0 {
                        self.select_rect.set_corner( 2, px, py);
                    }else if self.select_rect.mode==1{
                        if self.select_rect.sel_corner!=-1 {
                            self.select_rect.set_corner(self.select_rect.sel_corner,px,py);
                        }else{
                            let mdx = tmx  - self.select_rect.mouse_start_x;
                            let mdy = tmy  - self.select_rect.mouse_start_y;
                            let (dx,dy) = self.mouse_to_pixel_float( mdx, mdy);
                            if dx!=0.0 || dy!=0.0 {
                                let mut left = (self.select_rect.sav_left as f64) + dx;
                                let mut top = (self.select_rect.sav_top as f64) + dy;
                                let mut right = (self.select_rect.sav_right as f64) + dx;
                                let mut bottom = (self.select_rect.sav_bottom as f64) + dy;
                                // Prevent the rectangle to go out limits
                                if (left<0.0)||(right>=(self.nb_pixs_columns as f64)){
                                    left = self.select_rect.left as f64;
                                    right = self.select_rect.right as f64;
                                }
                                if (top<0.0)||(bottom>=(self.nb_pixs_rows as f64)){
                                    top = self.select_rect.top as f64;
                                    bottom = self.select_rect.bottom as f64;
                                }

                                self.select_rect.set_corner(0, left as i32, top as i32);
                                self.select_rect.set_corner(2, right as i32, bottom as i32);

                            }
                        }

                    }

                    if self.select_rect.is_empty()==false {

                        self.restore_sprite();

                        let (x0,x1) =   if self.select_rect.right > self.select_rect.left {
                                            (self.select_rect.left,self.select_rect.right)
                                        }else{
                                            (self.select_rect.right,self.select_rect.left)
                                        };
                        let (y0,y1) =   if self.select_rect.top > self.select_rect.bottom {
                                            (self.select_rect.top,self.select_rect.bottom)
                                        }else{
                                            (self.select_rect.bottom,self.select_rect.top)
                                        };

                        if state.contains(gdk::ModifierType::CONTROL_MASK){
                            // Fill Rectangle
                            self.fill_rectangle(&self.sprite, x0, y0, x1, y1, draw_color);
                        }else{
                            // Draw Rectangle
                            self.draw_rectangle(&self.sprite, x0, y0, x1, y1, draw_color);
                        }
                        _widget.queue_draw();
                        _widget.emit("edit-changed",&[]);
                    }  
        
                    //println!("Select Mode : Button Motion x={} y={}", x, y);
                }

            }

        }
       
    }
}