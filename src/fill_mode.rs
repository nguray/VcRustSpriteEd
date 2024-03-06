//use gio::prelude::*;
use gtk::prelude::*;
use gdk_pixbuf::*;

use crate::edit_area::{EditAreaDatas,UndoMode};
use crate::rgb_utils::{*};

use crate::rpoint::{RPoint};

pub trait FillMode {

    fn floodFillGetColor(&self, sprite: &Pixbuf, x: i32, y: i32)->(u32,bool);
    fn flood_fill(&self, sprite: &Pixbuf, fill_x: i32, fill_y: i32, fill_color: u32);
    fn init_fill_mode(&mut self);
    fn draw_fill_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context);
    fn button_press_event_fill_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn button_release_event_fill_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton);
    fn motion_notify_event_fill_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion);

}

impl FillMode for EditAreaDatas {

    fn floodFillGetColor(&self, sprite: &Pixbuf, x: i32, y: i32)->(u32,bool)
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

    fn flood_fill(&self, sprite: &Pixbuf, fill_x: i32, fill_y: i32, fill_color: u32)
    {
        //-----------------------------------------------------------------------
    
        // g_assert(gdk_pixbuf_get_colorspace (pixbuf) == GDK_COLORSPACE_RGB); // 
        // g_assert(gdk_pixbuf_get_bits_per_sample (pixbuf) == 8);              //  
        // g_assert(gdk_pixbuf_get_has_alpha (pixbuf));                         //
        // g_assert(n_channels == 4);
    
        let (target_color,res) = self.floodFillGetColor( &sprite, fill_x, fill_y);
        let (red,green,blue,alpha) = get_rgba(fill_color as u32);

        if res==false || (target_color==fill_color) {
             return;
        }

        let width  = sprite.get_width();  
        let height = sprite.get_height(); 
    
        //-- Créer la pile
        let mut stk:Vec<RPoint> = Vec::new();

        let pt = RPoint{x: fill_x,y: fill_y};
        stk.push(pt);
    
        let mut x: i32;
        let mut y: i32;
        let mut fStartNord: bool = false;
        let mut fStartSud: bool = false;
        let mut xStartLine: i32 = 0;
        let mut xEndLine: i32 = 0;
        let mut fNord = false;
        let mut fSud = false;

        while stk.len()!=0 {

            let (mut start_x,mut start_y) = match stk.pop(){
                            Some(pt) => {
                                (pt.x,pt.y)
                            }
                            None => {
                                (0,0)
                            }
                        };
    
    
            //-- Vérifier au Nord
            fStartNord = false;
    
            if  start_y>0 {
                let (pix_color,f) = self.floodFillGetColor( &sprite, start_x, start_y-1);
                if f==true {
                    if pix_color==target_color {
                        let pt = RPoint{x: start_x, y: start_y-1};
                        stk.push(pt);
                        fStartNord = true;
                    }
                }

            }
    
            //-- Vérifier au sud
            fStartSud = false;
            if start_y<(height-1) {
                let (pix_color,f) = self.floodFillGetColor( &sprite, start_x, start_y+1);
                if f==true {
                    if pix_color==target_color {
                        let pt = RPoint{x: start_x, y: start_y+1};
                        stk.push(pt);
                        fStartSud = true;
                    }
                }
            }

            y = start_y;

            //-- Aller vers l'est
            xEndLine = start_x;
            fNord = fStartNord;
            fSud = fStartSud;
            if xEndLine<(width-1){

                x = xEndLine + 1;
                loop{
                    let (pix_color,f) = self.floodFillGetColor( &sprite, x, y);
                    if f==false || pix_color!=target_color {
                        break;
                    }

                    //-- Vérifier au Nord
                    let (pix_color,f) = self.floodFillGetColor( &sprite, x, y-1);
                    if f==true && y>0 {
                        if target_color==pix_color {
                            if (fNord==false){
                                let pt = RPoint{x: x, y: y-1};
                                stk.push(pt);            
                                fNord = true;        
                            }
                        }else{
                            fNord = false;
                        }
                    }else{
                        fNord = false;
                    }

                    //-- Vérifier au sud
                    let (pix_color,f) = self.floodFillGetColor( &sprite, x, y+1);
                    if y<(height-1) && f==true {
                        if target_color==pix_color {
                            if fSud==false {
                                let pt = RPoint{x: x, y: y+1};
                                stk.push(pt);            
                                fSud = true;        
                            }
                        }else{
                            fSud = false;
                        }
                    }else{
                        fSud = false;
                    }

                    xEndLine = x;
                    x = x + 1;
                    if x>=width {
                        break;
                    }
                }
    
            }else{
                xEndLine = width-1;

            }
    
            //-- Aller vers l'ouest
            xStartLine = start_x;
            fNord = fStartNord;
            fSud = fStartSud;
            if xStartLine>0 {
                
                x = xStartLine - 1;

                loop{

                    let (pix_color,f) = self.floodFillGetColor( &sprite, x, y);
                    if f==false || pix_color!=target_color {
                        break;
                    }

                    //-- Vérifier au Nord
                    let (pix_color,f) = self.floodFillGetColor( &sprite, x, y-1);
                    if f==true && y>0 {
                        if target_color==pix_color {
                            if (fNord==false){
                                let pt = RPoint{x: x, y: y-1};
                                stk.push(pt);            
                                fNord = true;        
                            }
                        }else{
                            fNord = false;
                        }
                    }else{
                        fNord = false;
                    }

                    //-- Vérifier au sud
                    let (pix_color,f) = self.floodFillGetColor( &sprite, x, y+1);
                    if y<(height-1) && f==true {
                        if target_color==pix_color {
                            if fSud==false {
                                let pt = RPoint{x: x, y: y+1};
                                stk.push(pt);            
                                fSud = true;        
                            }
                        }else{
                            fSud = false;
                        }
                    }else{
                        fSud = false;
                    }

                    xStartLine = x;
                    x = x - 1;
                    if x<0 {
                        break;
                    }
                }

            }else{
                xStartLine = 0;
            }
            //-- Tracer la line
            for x in xStartLine..=xEndLine {
                sprite.put_pixel( x as u32, y as u32, red, green, blue, alpha);

            }
    
        }
    
    }

    fn init_fill_mode(&mut self) {
        self.select_rect.init();
    }

    fn draw_fill_mode(&mut self, _widget: &gtk::Widget, cr: &cairo::Context) 
    {
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

        //self.draw_select_rect(&cr);

    }

    fn button_press_event_fill_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) 
    {
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
                //--
                self.backup_sprite();
                self.undo_mode = UndoMode::FILL;
                self.flood_fill( &self.sprite, px, py, draw_color);
                //--
                _widget.queue_draw();
                _widget.emit("edit-changed",&[]);

            }
        }
    }

    fn button_release_event_fill_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventButton) 
    {
        //---------------------------------------------------

    }

    fn motion_notify_event_fill_mode(&mut self, _widget: &gtk::Widget, event: &gdk::EventMotion) 
    {
        //---------------------------------------------------

    }

}