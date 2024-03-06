//use gio::prelude::*;
use gtk::prelude::*;
use gdk_pixbuf::*;

use crate::edit_area::EditAreaDatas;
use crate::rgb_utils::{*};
use crate::rrect::{RRect};

pub trait SelectMode {

    fn copy_select(&mut self);
    fn paste_select(&mut self);
    fn cut_select(&mut self);
    fn init_select_mode(&mut self);
    fn hit_handle(&self, mx: f64, my: f64)->i32;
    fn in_select_rect(&self, mx: f64, my: f64)->bool;
    fn draw_select_rect(&self, cr: &cairo::Context);
    fn draw_select_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context);
    fn button_press_event_select_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn button_release_event_select_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn motion_notify_event_select_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion);

}

impl SelectMode for EditAreaDatas {

    fn copy_select(&mut self){
        //-----------------------------------------
        if self.select_rect.is_empty()==false {
            self.sprite.copy_area( self.select_rect.left, self.select_rect.top,
                    self.select_rect.width(), self.select_rect.height(), &self.sprite_copy,0,0);
            self.copy_rect.left = self.select_rect.left;
            self.copy_rect.right = self.select_rect.right;
            self.copy_rect.top = self.select_rect.top;
            self.copy_rect.bottom = self.select_rect.bottom;
            self.select_rect.empty();
        }
    }

    fn paste_select(&mut self){
        //-----------------------------------------
        self.select_rect.left = self.copy_rect.left;
        self.select_rect.right = self.copy_rect.right;
        self.select_rect.top = self.copy_rect.top;
        self.select_rect.bottom = self.copy_rect.bottom;
        self.select_rect.mode = 2;
        self.backup_sprite();
        self.sprite_copy.copy_area( 0, 0, self.select_rect.width(), self.select_rect.height(),
                            &self.sprite,self.select_rect.left,self.select_rect.top);
    }

    fn cut_select(&mut self){
        //-----------------------------------------
        if self.select_rect.is_empty()==false {
            self.sprite.copy_area( self.select_rect.left, self.select_rect.top,
                    self.select_rect.width(), self.select_rect.height(), &self.sprite_copy, 0, 0);
            self.fill_rect();
            self.copy_rect.left = self.select_rect.left;
            self.copy_rect.right = self.select_rect.right;
            self.copy_rect.top = self.select_rect.top;
            self.copy_rect.bottom = self.select_rect.bottom;
            self.select_rect.empty();
        }
    }

    fn init_select_mode(&mut self) {
        self.select_rect.init();

    }

    fn hit_handle(&self, mx: f64, my: f64)->i32 {
        let mut id: i32 = -1;
        for i in 0..4 {
            let ( px, py) = self.select_rect.get_corner(i);
            let ( x, y) = self.pixel_to_mouse(px,py);
            if (mx>=x) && mx<=(x+(self.cell_size as f64)) &&
                    (my>=y) && my<=(y+(self.cell_size as f64)) {
                id = i;
                break;
            }
        }
        id
    }

    fn in_select_rect(&self, mx: f64, my: f64)->bool {
        let ( px1, py1) = self.select_rect.get_corner(0);
        let ( x1, y1) = self.pixel_to_mouse(px1,py1);
        let ( px2, py2) = self.select_rect.get_corner(2);
        let ( x2, y2) = self.pixel_to_mouse(px2,py2);
        let (mut _x1,mut _x2) = if x2>x1 {
            (x1,x2)
        }else{
            (x2,x1)
        };
        let (mut _y1,mut _y2) = if y2>y1 {
            (y1,y2)
        }else{
            (y2,y1)
        };
        _y2 += self.cell_size as f64;
        _x2 += self.cell_size as f64;
        (mx>=_x1)&&(mx<=_x2)&&(my>=_y1)&&(my<=_y2)
    }

    fn draw_select_rect(&self, cr: &cairo::Context) {

        if self.select_rect.is_empty()==false {
            //--
            let ( px1, py1) = self.select_rect.get_corner(0);
            let ( px2, py2) = self.select_rect.get_corner(2);
            let ( mut x1, mut y1) = self.pixel_to_mouse(px1,py1);
            let ( mut x2, mut y2) = self.pixel_to_mouse(px2,py2);

            if x1>x2{
                std::mem::swap(&mut x1, &mut x2);
            }
            // let (mut _x1,mut _x2) = if x2>x1 {
            //     (x1,x2)
            // }else{
            //     (x2,x1)
            // };
            if y1>y2{
                std::mem::swap(&mut y1, &mut y2);
            }
            // let (mut _y1,mut _y2) = if y2>y1 {
            //     (y1,y2)
            // }else{
            //     (y2,y1)
            // };

            //-- Draw Handles
            cr.set_source_rgba(0.0,0.0,1.0,1.0);
            let dx = (self.cell_size - 5) as f64;
            let x = x1 + 2.0;
            let y = y1 + 2.0;
            cr.rectangle(x,y,dx,dx);
            cr.fill();
            let x = x2 + 2.0;
            let y = y1 + 2.0;
            cr.rectangle(x,y,dx,dx);
            cr.fill();
            let x = x2 + 2.0;
            let y = y2 + 2.0;
            cr.rectangle(x,y,dx,dx);
            cr.fill();
            let x = x1 + 2.0;
            let y = y2 + 2.0;
            cr.rectangle(x,y,dx,dx);
            cr.fill();

            //-- Draw Frame
            y2 += self.cell_size as f64;
            x2 += self.cell_size as f64;
            cr.set_source_rgba(0.0,0.0,1.0,0.1);
            cr.rectangle(x1,y1,x2-x1,y2-y1);
            cr.fill();
        }

    }

    fn draw_select_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context) {
        
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

        cr.set_source_rgba( 0.0, 0.0, 0.0, 1.0);
        self.draw_frame( &_widget, &cr);

        self.draw_pixels( &cr);

        self.draw_select_rect(&cr);

    }    

    fn button_press_event_select_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {

        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let state: gdk::ModifierType = event.get_state();
            //let widget1 = self.get_instance();
            //let b: bool = state.contains(CONTROL_MASK);

            let (px,py) = self.mouse_to_pixel( tmx, tmy);
            if self.in_edit_area(px,py){

                if self.select_rect.mode==0 {
                    //--
                    self.select_rect.set_corner( 0, px, py);
                    self.select_rect.set_corner( 2, px, py);
                    _widget.queue_draw();

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
                        self.select_rect.mode = 0;
                        self.select_rect.set_corner( 0, px, py);
                        self.select_rect.set_corner( 2, px, py);
                        self.select_rect.sel_corner = -1;
                        _widget.queue_draw();
                        
                    }

                }else if self.select_rect.mode==2 {
                    if self.in_select_rect( tmx, tmy)==true {
                        //-- Start Move Select Rect
                        self.select_rect.mouse_start_x = tmx;
                        self.select_rect.mouse_start_y = tmy;
                        self.select_rect.backup_position();
                        _widget.queue_draw();

                    }else{
                        self.select_rect.mode = 0;
                        self.select_rect.set_corner( 0, px, py);
                        self.select_rect.set_corner( 2, px, py);
                        self.select_rect.sel_corner = -1;
                        self.copy_rect.empty();
                        _widget.queue_draw();

                    }
                }

            }

        }

        //let mx: u32 = x as u32;
        //let my: u32 = y as u32;
        //_widget.emit("left-clicked", &[&mx,&my]);

    }

    fn button_release_event_select_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {
        //--
        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let (px,py) = self.mouse_to_pixel( tmx, tmy);
            self.last_pixel_x = px;
            self.last_pixel_y = py;
            //println!("Release px={} py={}", px, py);
        }

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
        }
        //--
        self.select_rect.sel_corner = -1;

    }

    fn motion_notify_event_select_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion) {
        //---
        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let state: gdk::ModifierType = event.get_state();

            //let widget1 = self.get_instance();
            //let f:bool = state.contains(gdk::ModifierType::BUTTON1_MASK);
            //if state.contains(gdk::ModifierType::BUTTON1_MASK) && state.contains(gdk::ModifierType::CONTROL_MASK) {
            if state.contains(gdk::ModifierType::CONTROL_MASK){
                //--

            }else{

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
                            let (dx,dy) = self.mouse_to_pixel( mdx, mdy);
                            let left = self.select_rect.sav_left + dx;
                            let top = self.select_rect.sav_top + dy;
                            let right = self.select_rect.sav_right + dx;
                            let bottom = self.select_rect.sav_bottom + dy;
                            let mut rect = RRect::new(left as i32,top as i32,right as i32,bottom as i32);
                            if rect.left<0 {
                                rect.offset(-rect.left,0);
                            }else if rect.right>=self.nb_pixs_columns {
                                rect.offset(self.nb_pixs_columns-rect.right-1,0);
                            }
                            if rect.top<0 {
                                rect.offset( 0, -rect.top);
                            }else if rect.bottom>=self.nb_pixs_rows {
                                rect.offset( 0, self.nb_pixs_rows-rect.bottom-1);
                            }
                            self.select_rect.set_corner(0, rect.left , rect.top);
                            self.select_rect.set_corner(2, rect.right, rect.bottom);

                        }
                    }else if self.select_rect.mode==2 {
                        let mdx = tmx  - self.select_rect.mouse_start_x;
                        let mdy = tmy  - self.select_rect.mouse_start_y;
                        let (dx,dy) = self.mouse_to_pixel( mdx, mdy);

                        let left = self.select_rect.sav_left + dx;
                        let top = self.select_rect.sav_top + dy;
                        let right = self.select_rect.sav_right + dx;
                        let bottom = self.select_rect.sav_bottom + dy;

                        // Prevent the rectangle to go out limits
                        let mut rect = RRect::new(left as i32,top as i32,right as i32,bottom as i32);
                        if rect.left<0 {
                            rect.offset(-rect.left,0);
                        }else if rect.right>=self.nb_pixs_columns {
                            rect.offset(self.nb_pixs_columns-rect.right-1,0);
                        }
                        if rect.top<0 {
                            rect.offset( 0, -rect.top);
                        }else if rect.bottom>=self.nb_pixs_rows {
                            rect.offset( 0, self.nb_pixs_rows-rect.bottom-1);
                        }
                        self.select_rect.set_corner(0, rect.left , rect.top);
                        self.select_rect.set_corner(2, rect.right, rect.bottom);

                        self.restore_sprite();
                        self.sprite_copy.copy_area( 0, 0, self.select_rect.width(), self.select_rect.height(),
                                            &self.sprite,self.select_rect.left,self.select_rect.top);
                
                    }
                    _widget.queue_draw();
                    _widget.emit("edit-changed",&[]);

                    //println!("Select Mode : Button Motion x={} y={}", x, y);
                }
            }
        }

    }

}