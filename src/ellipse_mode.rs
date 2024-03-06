//use gio::prelude::*;
use gtk::prelude::*;
use gdk_pixbuf::*;

use crate::edit_area::{EditAreaDatas,UndoMode};
use crate::rgb_utils::{*};
use crate::select_mode::SelectMode;

pub trait EllipseMode {
    fn draw_ellipse(&self, sprite: &Pixbuf, startX: i32, startY: i32, endX: i32, endY: i32, col: u32, fFill : bool);
    fn fill_ellipse(&self, sprite: &Pixbuf, left: i32, top: i32, right: i32, bottom: i32, col: u32);
    fn init_ellipse_mode(&mut self);
    fn draw_ellipse_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context);
    fn button_press_event_ellipse_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn button_release_event_ellipse_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn motion_notify_event_ellipse_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion);
    fn draw_horizontal_line(&self, sprite: &Pixbuf, xLeft: i32, xRight: i32, y: i32, col: u32);
    fn border_ellipse(&self, sprite: &Pixbuf, left: i32, top: i32, right: i32, bottom: i32, col: u32); 

}

impl EllipseMode for EditAreaDatas{

    fn draw_ellipse(&self, sprite: &Pixbuf, startX: i32, startY: i32, endX: i32, endY: i32, col: u32, fFill : bool)
    {
        //----------------------------------------------------------------------
        let (_startX,_endX) =   if endX<startX {
                                    (endX,startX)
                                }else{
                                    (startX,endX)
                                };

        let (_startY,_endY) =   if endY<startY {
                                    (endY,startY)
                                }else{
                                    (startY,endY)
                                };

        if (_endX!=_startX) || (_endY!=_startY) {
            //-- Tracer l'Ellipse
            if (fFill){
                self.fill_ellipse(sprite, _startX, _startY, _endX, _endY, col);
            }else{
                self.border_ellipse(sprite, _startX, _startY, _endX, _endY, col);
            }
        }
    
    }    

    fn draw_horizontal_line(&self, sprite: &Pixbuf, xLeft: i32, xRight: i32, y: i32, col: u32)
    {
        //---------------------------------------------
        let (red,green,blue,alpha) = get_rgba(col);            
    
        for x in xLeft..=xRight {

            sprite.put_pixel( x as u32, y as u32, red, green, blue, alpha);

        }
    
    }

    fn fill_ellipse(&self, sprite: &Pixbuf, left: i32, top: i32, right: i32, bottom: i32, col: u32)
    {
        //---------------------------------------------
        let a = (right - left) / 2;
        let b = (bottom - top) / 2;
    
        let mut x: i32 = 0;
        let mut y: i32 = b;
    
        let a2 = a * a;
        let b2 = b * b;
        let a2b2 = a2 + b2;
        let a2sqr = a2 + a2;
        let b2sqr = b2 + b2;
        let a4sqr = a2sqr + a2sqr;
        let b4sqr = b2sqr + b2sqr;
        let a8sqr = a4sqr + a4sqr;
        let b8sqr = b4sqr + b4sqr;
        let a4sqr_b4sqr = a4sqr + b4sqr;
    
        let mut _fn = a8sqr + a4sqr;
        let _fnn = a8sqr;
        let _fnnw = a8sqr;
        let mut _fnw = a8sqr + a4sqr - b8sqr * a + b8sqr;
        let _fnwn = a8sqr;
        let _fnwnw = a8sqr + b8sqr;
        let _fnww = b8sqr;
        let _fwnw = b8sqr;
        let _fww = b8sqr;
        let mut d1 = b2 - b4sqr * a + a4sqr;
    
        while (_fnw < a2b2) || (d1 < 0) || (((_fnw - _fn) > b2) && (y > 0)) {
    
            self.draw_horizontal_line( &sprite, left + x, right - x, top + y, col);
            //DrawHorizontalLine( pixbuf, left + x, right - x, top + y, col);
            self.draw_horizontal_line( &sprite, left + x, right - x, bottom - y, col);
            //DrawHorizontalLine( pixbuf, left + x, right - x, bottom - y, col);
    
            y -= 1;
            if (d1 < 0) || ((_fnw - _fn) > b2) {
                d1 = d1 + _fn;
                _fn = _fn + _fnn;
                _fnw = _fnw + _fnwn;
            }else{
                x += 1;
                d1 = d1 + _fnw;
                _fn = _fn + _fnnw;
                _fnw = _fnw + _fnwnw;
            }

        }
    
        let mut _fw = _fnw - _fn + b4sqr;
        let mut d2 = d1 + (_fw + _fw - _fn - _fn + a4sqr_b4sqr + a8sqr) / 4;
        _fnw = _fnw + (b4sqr - a4sqr);
    
        let mut old_y = y + 1;
    
        while x <= a {
            if y != old_y {
                self.draw_horizontal_line( &sprite, left + x, right - x, top + y, col);
                //DrawHorizontalLine( pixbuf, left + x, right - x, top + y, col);
                self.draw_horizontal_line( &sprite, left + x, right - x, bottom - y, col);
                //DrawHorizontalLine( pixbuf, left + x, right - x, bottom - y, col);              
            }
            old_y = y;

            x += 1;
            if d2<0 {
                y -= 1;
                d2 = d2 + _fnw;
                _fw = _fw + _fwnw;
                _fnw = _fnw + _fnwnw;
            }else{
                d2 = d2 + _fw;
                _fw = _fw + _fww;
                _fnw = _fnw + _fnww;
            }
    
        }
    
    }    


    fn border_ellipse(&self, sprite: &Pixbuf, left: i32, top: i32, right: i32, bottom: i32, col: u32) 
    {
        //---------------------------------------------
        let a = (right - left) / 2;
        let b = (bottom - top) / 2;

        let mut x: i32 = 0;
        let mut y: i32 = b;

        let a2 = a * a;
        let b2 = b * b;
        let a2b2 = a2 + b2;
        let a2sqr = a2 + a2;
        let b2sqr = b2 + b2;
        let a4sqr = a2sqr + a2sqr;
        let b4sqr = b2sqr + b2sqr;
        let a8sqr = a4sqr + a4sqr;
        let b8sqr = b4sqr + b4sqr;
        let a4sqr_b4sqr = a4sqr + b4sqr;
    
        let mut _fn = a8sqr + a4sqr;
        let _fnn = a8sqr;
        let _fnnw = a8sqr;
        let mut _fnw = a8sqr + a4sqr - b8sqr * a + b8sqr;
        let _fnwn = a8sqr;
        let _fnwnw = a8sqr + b8sqr;
        let _fnww = b8sqr;
        let _fwnw = b8sqr;
        let _fww = b8sqr;
        let mut d1 = b2 - b4sqr * a + a4sqr;

        let (red,green,blue,alpha) = get_rgba(col);            

        while ((_fnw < a2b2) || (d1 < 0) || (((_fnw - _fn) > b2) && (y > 0))) {

            //put_pixel(pixbuf, left + x, top + y, red, green, blue, alpha);
            sprite.put_pixel( (left + x) as u32, (top + y) as u32, red, green, blue, alpha);
            //put_pixel(pixbuf, right - x, top + y, red, green, blue, alpha);
            sprite.put_pixel( (right - x) as u32, (top + y) as u32, red, green, blue, alpha);
            //put_pixel(pixbuf, left + x, bottom - y, red, green, blue, alpha);
            sprite.put_pixel( (left + x) as u32, (bottom - y) as u32, red, green, blue, alpha);
            //put_pixel(pixbuf, right - x, bottom - y, red, green, blue, alpha);
            sprite.put_pixel( (right - x) as u32, (bottom - y) as u32, red, green, blue, alpha);


            y -= 1;
            if ((d1 < 0) || ((_fnw - _fn) > b2)) {
                d1 += _fn;
                _fn += _fnn;
                _fnw += _fnwn;
            } else {
                x += 1;
                d1 += _fnw;
                _fn += _fnnw;
                _fnw += _fnwnw;
            }
        }

        let mut _fw = _fnw - _fn + b4sqr;
        let mut d2 = d1 + (_fw + _fw - _fn - _fn + a4sqr_b4sqr + a8sqr) / 4;
        _fnw = _fnw + (b4sqr - a4sqr);
    
        while (x <= a) {

            //put_pixel(pixbuf, left + x, top + y, red, green, blue, alpha);
            sprite.put_pixel( (left + x) as u32, (top + y) as u32, red, green, blue, alpha);
            //put_pixel(pixbuf, right - x, top + y, red, green, blue, alpha);
            sprite.put_pixel( (right - x) as u32, (top + y) as u32, red, green, blue, alpha);
            //put_pixel(pixbuf, left + x, bottom - y, red, green, blue, alpha);
            sprite.put_pixel( (left + x) as u32, (bottom - y) as u32, red, green, blue, alpha);
            //put_pixel(pixbuf, right - x, bottom - y, red, green, blue, alpha);
            sprite.put_pixel( (right - x) as u32, (bottom - y) as u32, red, green, blue, alpha);

            x += 1;
            if (d2 < 0) {
                y -= 1;
                d2 += _fnw;
                _fw += _fwnw;
                _fnw += _fnwnw;
            } else {
                d2 += _fw;
                _fw += _fww;
                _fnw += _fnww;
            }
        }

    }

    fn init_ellipse_mode(&mut self) {
        self.select_rect.init();
    }

    fn draw_ellipse_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context) {
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

    fn button_press_event_ellipse_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {

        if let Some((x, y)) = event.get_coords() {

            let tmx = x - (self.origin_x as f64);
            let tmy = y - (self.origin_y as f64);

            let state: gdk::ModifierType = event.get_state();
            //let widget1 = self.get_instance();
            //let b: bool = state.contains(CONTROL_MASK);
            // let draw_color  =   if event.get_button()==1 {
            //                         self.foreground_color
            //                     }else if event.get_button()==3 {
            //                         self.background_color
            //                     }else{
            //                         self.foreground_color       
            //                     };
            let (px,py) = self.mouse_to_pixel( tmx, tmy);
            if self.in_edit_area(px,py){

                if self.select_rect.mode==0 {
                    //--
                    self.backup_sprite();
                    self.undo_mode = UndoMode::ELLIPSE;
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

                //let mx: u32 = x as u32;
                //let my: u32 = y as u32;
                //_widget.emit("left-clicked", &[&mx,&my]);
                _widget.queue_draw();
                _widget.emit("edit-changed",&[]);
            }
        }
                
    }

    fn button_release_event_ellipse_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) {
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

    fn motion_notify_event_ellipse_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion) {

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
                                            (self.select_rect.bottom,self.select_rect.top)
                                        }else{
                                            (self.select_rect.top,self.select_rect.bottom)
                                        };

                        if state.contains(gdk::ModifierType::CONTROL_MASK){
                            // Fill Rectangle
                            //self.draw_ellipse(&self.sprite, x0, y0, x1, y1, draw_color, true);
                            self.fill_ellipse(&self.sprite, x0, y0, x1, y1, draw_color);
                        }else{
                            // Draw Rectangle
                            //self.draw_ellipse(&self.sprite, x0, y0, x1, y1, draw_color, false);
                            self.border_ellipse(&self.sprite, x0, y0, x1, y1, draw_color);
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