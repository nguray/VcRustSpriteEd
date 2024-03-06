//! 
//!
//! 
//! 
//!

use std::mem::transmute;
use glib::Type;
use glib::subclass;
use glib::translate::*;
use std::cell::{Cell, RefCell};
use gdk_pixbuf::*;
//use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::rc::Rc;

use gtk::Clipboard;

use gdk::prelude::GdkContextExt;
use gdk::{RGBA, WindowExt, Cursor, Display, keys};


use super::rgb_utils::{*};

use crate::rpoint::{RPoint};

use crate::rrect::{RRect};
use crate::select_rect::{SelectRect};

use crate::pencil_mode::PencilMode;
use crate::rectangle_mode::RectangleMode;
use crate::ellipse_mode::EllipseMode;
use crate::select_mode::SelectMode;
use crate::fill_mode::FillMode;

const MINIMUM_WIDTH: i32 = 640;
const MINIMUM_HEIGHT: i32 = 480;


#[derive(PartialEq)]
pub enum EditMode {
    SELECT,
    PENCIL,
    RECTANGLE,
    ELLIPSE,
    FILL,
}

#[derive(PartialEq, Copy, Clone)]
pub enum UndoMode {
    NONE,
    PENCIL,
    RECTANGLE,
    ELLIPSE,
    FILL,
    FLIP_HORIZONTALY,
    FLIP_VERTICALY,
    SWING_RIGHT,
    SWING_LEFT,
}

//#[derive( Copy, Clone)]
pub struct EditAreaDatas {
    pub nb_pixs_columns: i32,
    pub nb_pixs_rows: i32,
    pub cell_size: i32,
    pub origin_x: i32,
    pub origin_y: i32,
    pub m_scale: f32,
    pub start_origin_x: i32,
    pub start_origin_y: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub sprite: Rc<Pixbuf>,
    pub sprite_bak: Pixbuf,
    pub sprite_copy: Pixbuf,
    pub file_name: String,
    pub foreground_color: u32,
    pub background_color: u32,
    pub last_pixel_x: i32,
    pub last_pixel_y: i32,
    pub mode : EditMode,
    pub copy_rect: SelectRect,
    pub select_rect: SelectRect,
    pub undo_mode : UndoMode,
    //pub pick_cursor : gdk::Cursor,
    draw: fn (editpriv: &mut EditAreaDatas, _widget: &gtk::Widget, cr: &cairo::Context),
    button_press_event: fn (editpriv: &mut EditAreaDatas, _widget: &gtk::Widget, event: &gdk::EventButton),
    button_release_event: fn (editpriv: &mut EditAreaDatas, _widget: &gtk::Widget, event: &gdk::EventButton),
    motion_notify_event: fn (editpriv: &mut EditAreaDatas, _widget: &gtk::Widget, event: &gdk::EventMotion),

}

impl EditAreaDatas {

    fn new() -> Self {
        Self {
            nb_pixs_columns: 32,
            nb_pixs_rows: 32,
            cell_size: 10,
            origin_x: 0,
            origin_y: 0,
            m_scale: 1.0f32,
            start_origin_x: 0,
            start_origin_y: 0,
            start_x: 0,
            start_y: 0,
            sprite: Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
            sprite_bak: Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap(),
            sprite_copy: Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap(),
            file_name: String::from(""),
            foreground_color: 0x00FF00FF,
            background_color: 0x00,
            last_pixel_x: -1,
            last_pixel_y: -1,
            mode : EditMode::SELECT,
            copy_rect: SelectRect::new(0,0,0,0),
            select_rect: SelectRect::new(0,0,0,0),
            undo_mode : UndoMode::NONE,
            //pick_cursor : gdk::Cursor::from_pixbuf(&gdk::Display::get_default().unwrap(), &Pixbuf::from_resource("/res/Swing90RightIcon.png").unwrap(), 0, 0),
            draw: EditAreaDatas::draw_select_mode,
            button_press_event: EditAreaDatas::button_press_event_select_mode,
            button_release_event: EditAreaDatas::button_release_event_select_mode,
            motion_notify_event: EditAreaDatas::motion_notify_event_select_mode,
        }

    }

    pub fn compute_cell_size(&mut self, win_width: i32, win_height: i32){
        //---------------------------------------------------------
        //--
        //let mut p = self.priv_.borrow_mut();
        //-- Calculer la taille du pixel affiché
        let o1 = (win_width-4) / self.nb_pixs_columns;
        let o2 = (win_height-4) / self.nb_pixs_rows;
        if o1<o2 {
            self.cell_size = ((o1 as f32) * self.m_scale) as i32;
        }else{
            self.cell_size = ((o2 as f32) * self.m_scale) as i32;
        }

    }

    pub fn draw_frame(&self, _widget: &gtk::Widget, cr: &cairo::Context){
        //---------------------------------------------------------
        //--
        //let w = _widget.get_allocation().width - 1;
        //let h = _widget.get_allocation().height - 1;

        let nb_cols = self.nb_pixs_columns;
        let nb_rows = self.nb_pixs_rows;
        let cell_size= self.cell_size;

        for row in 0..nb_rows+1 {
            let y = (row * cell_size + 4) as f64;
            for col in 0..nb_cols+1 {
                let x = (col * cell_size + 4) as f64;
                cr.move_to(x-0.5, y);
                cr.line_to(x+0.5, y);
            }
        }
        cr.stroke();

        //-- Draw sprite
        let x = self.cell_size * (self.nb_pixs_columns+2);
        cr.set_source_pixbuf(&self.sprite, x as f64, 4f64);
        cr.paint();

        //-- Draw sprite backup
        //let x = self.cell_size * (self.nb_pixs_columns+2);
        cr.set_source_pixbuf(&self.sprite_bak, x as f64, 48f64);
        cr.paint();

        //-- Draw sprite copy
        //let x = self.cell_size * (self.nb_pixs_columns+2);
        cr.set_source_pixbuf(&self.sprite_copy, x as f64, 88f64);
        cr.paint();

        // Drawing code goes here
        // cr.set_line_width(0.5);
        // cr.set_source_rgb(0.0, 0.0, 0.0);
        // cr.rectangle(1.0,1.0, w.into(), h.into());
        // cr.move_to(1.0, 1.0);
        // cr.line_to(w as f64, h as f64);
        // cr.move_to(w as f64, 1.0);
        // cr.line_to(1.0, h as f64);
        // cr.stroke();

    }

    pub fn in_edit_area(&self,x: i32,y: i32)->bool{
        //---------------------------------------------------------
        (x>=0)&&(x<self.nb_pixs_columns)&&(y>=0)&&(y<self.nb_pixs_rows)

    } 

    pub fn draw_pixels(&self, cr: &cairo::Context) {
        //---------------------------------------------------------
        unsafe {
            let row_stride = self.sprite.get_rowstride();
            let n_channels = self.sprite.get_n_channels();
            let pixels = self.sprite.get_pixels();
            let dx = (self.cell_size - 2) as f64;
            let mut x: f64;
            let mut y: f64;
            let mut ip: usize;
            let mut red: f64;
            let mut green: f64;
            let mut blue: f64;
            let mut alpha: f64;
            for iy in 0..self.nb_pixs_rows{
                for ix in 0..self.nb_pixs_columns{
                    ip = (iy * row_stride + ix * n_channels) as usize;
                    red     = (pixels[ip] as f64) / 255.0;
                    green   = (pixels[ip+1] as f64) / 255.0;
                    blue    = (pixels[ip+2] as f64) / 255.0;
                    alpha   = (pixels[ip+3] as f64) / 255.0;
                    cr.set_source_rgba(red,green,blue,alpha);
                    x = (ix * self.cell_size + 5) as f64;
                    y = (iy * self.cell_size + 5) as f64;
                    cr.rectangle(x, y, dx, dx);
                    cr.fill();
                }
            }
        }
    }   

    pub fn do_flip_horizontaly(&self){
        //---------------------------------------------------------
        unsafe {
            let row_stride = self.sprite.get_rowstride();
            let n_channels = self.sprite.get_n_channels();
            let pixelsDes = self.sprite.get_pixels();
            let pixelsSrc = self.sprite_bak.get_pixels();
            let mut ipSrc: usize;
            let mut ipDes: usize;
            let width = self.sprite.get_width();
            let height = self.sprite.get_height();
            let mut yOffset: i32;
            let w = width-1;
            for y in 0..height {
                yOffset = y * row_stride;
                for x in 0..width{
                    ipSrc = (yOffset + x * n_channels) as usize;
                    ipDes = (yOffset + (w-x) * n_channels) as usize;
                    pixelsDes[ipDes] = pixelsSrc[ipSrc];
                    pixelsDes[ipDes+1] = pixelsSrc[ipSrc+1];
                    pixelsDes[ipDes+2] = pixelsSrc[ipSrc+2];
                    pixelsDes[ipDes+3] = pixelsSrc[ipSrc+3];
                }
            }
        }

    }

    pub fn do_flip_verticaly(&self){
        //---------------------------------------------------------

        unsafe {
            let row_stride = self.sprite.get_rowstride();
            let n_channels = self.sprite.get_n_channels();
            let pixelsDes = self.sprite.get_pixels();
            let pixelsSrc = self.sprite_bak.get_pixels();
            let mut ipSrc: usize;
            let mut ipDes: usize;
            let width = self.sprite.get_width();
            let height = self.sprite.get_height();
            let mut ySrcOffset: i32;
            let mut yDesOffset: i32;
            let mut xOffset: i32;
            let h = height-1;
            for y in 0..height {
                ySrcOffset = y * row_stride;
                yDesOffset = (h-y) * row_stride;
                for x in 0..width{
                    xOffset = x * n_channels;
                    ipSrc = (ySrcOffset + xOffset) as usize;
                    ipDes = (yDesOffset + xOffset) as usize;
                    pixelsDes[ipDes] = pixelsSrc[ipSrc];
                    pixelsDes[ipDes+1] = pixelsSrc[ipSrc+1];
                    pixelsDes[ipDes+2] = pixelsSrc[ipSrc+2];
                    pixelsDes[ipDes+3] = pixelsSrc[ipSrc+3];
                }
            } 
        }

    }

    pub fn mouse_to_pixel(&self, mx: f64, my: f64)->(i32,i32) {
        //---------------------------------------------------------
        let x: i32 = (((mx as i32)-4) / self.cell_size) as i32;
        let y: i32 = (((my as i32)-4) / self.cell_size) as i32;
        return (x,y);
    }

    pub fn mouse_to_pixel_float(&self, mx: f64, my: f64)->(f64,f64) {
        //---------------------------------------------------------
        let x: f64 = (mx-4.0) / (self.cell_size as f64);
        let y: f64 = (my-4.0) / (self.cell_size as f64);
        return (x,y);
    }

    pub fn pixel_to_mouse(&self, x: i32, y: i32)->(f64,f64) {
        //---------------------------------------------------------
        let mx = (((x * self.cell_size) as f64) + 4.0) as f64;
        let my = (((y * self.cell_size) as f64) + 4.0) as f64;
        return (mx,my);
    }

    pub fn backup_sprite(&self) {
        //-------------------------------------------------------
        let w = self.sprite.get_width();
        let h = self.sprite.get_height();

        self.sprite.copy_area( 0, 0, w, h, &self.sprite_bak, 0, 0);
    
    }
    
    pub fn restore_sprite(&self)
    {
        //-------------------------------------------------------
        let w = self.sprite.get_width();
        let h = self.sprite.get_height();
        self.sprite_bak.copy_area( 0, 0, w, h, &self.sprite, 0, 0);
    
    }

    pub fn draw_line(&self, sprite: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32, c: u32) {
        //-----------------------------------------------------
        let width  = sprite.get_width();
        let height = sprite.get_height();

        if (x0>=0 && x0<width) && (y0>=0 && y0<height) &&
                (x1>=0 && x1<width) && (y1>=0 && y1<height) {

            let (r,g,b,a) = get_rgba(c);            
            
            let mut y: i32;
            let mut dum: i32;
            let ystep: i32;
            let mut error: i32;

            let mut _x0: i32 = x0;
            let mut _y0: i32 = y0;
            let mut _x1: i32 = x1;
            let mut _y1: i32 = y1;

            let steep = (y1-y0).abs() > (x1-x0).abs();
            if steep {
                dum = _x0;
                _x0  = _y0;
                _y0  = dum;
                dum = _x1;
                _x1  = _y1;
                _y1  = dum;
            }

            if _x0>_x1 {
                dum = _x0;
                _x0  = _x1;
                _x1  = dum;
                dum = _y0;
                _y0  = _y1;
                _y1  = dum;
            }
    
            let deltax = _x1 - _x0;
            let deltay = (_y1-_y0).abs();
            error  = deltax / 2;
    
            y  = _y0;
            if _y0 < _y1 {
                ystep = 1;
            }else{
                ystep = -1;
            }

            for x in _x0..=_x1 {
                if  steep {
                    self.sprite.put_pixel(y as u32, x as u32, r, g, b, a);
                }else{
                    self.sprite.put_pixel(x as u32, y as u32, r, g, b, a);
                }
                error = error - deltay;
                if error<0 {
                    y = y + ystep;
                    error = error + deltax;
                }
            }
    
        }
    }

    pub fn fill_rect(&mut self){
        if self.select_rect.is_empty()==false {
            for y in self.select_rect.top..=self.select_rect.bottom {
                for x in self.select_rect.left..=self.select_rect.right {
                    let (r,g,b,a) = get_rgba(self.background_color); 
                    self.sprite.put_pixel(x as u32,y as u32,r,g,b,a);
                }
            }
        }
    }

}

// This is the private part of our `EditAreaDatas` object.
// Its where state and widgets are stored when they don't
// need to be publicly accesible.
//#[derive(Debug)]
pub struct EditAreaPrivate {
    priv_: RefCell<EditAreaDatas>,
    //counter: Cell<u64>,

}

impl ObjectSubclass for EditAreaPrivate {
    const NAME: &'static str = "EditArea";
    type ParentType = gtk::DrawingArea;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.add_signal(
            "edit-changed",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[],
            Type::Unit,
        );
        klass.add_signal(
            "color-changed",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32,Type::U32],
            Type::Unit,
        );
        klass.add_signal(
            "sprite-transform",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[],
            Type::Unit,
        );
        klass.add_signal(
            "pick-foreground-color",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32],
            Type::Unit,
        );
        klass.add_signal(
            "pick-background-color",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32],
            Type::Unit,
        );

    }

    fn new() -> Self {
        Self {
            priv_: RefCell::new(EditAreaDatas::new()),
            //counter: Cell::new(0),
        }
    }

}

impl ObjectImpl for EditAreaPrivate {
    glib_object_impl!();

    // Here we are overriding the glib::Objcet::contructed
    // method. Its what gets called when we create our Object
    // and where we can initialize things.
    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);

        obj.downcast_ref::<gtk::Widget>().unwrap().add_events(
            gdk::EventMask::BUTTON_PRESS_MASK
                | gdk::EventMask::BUTTON_RELEASE_MASK
                | gdk::EventMask::BUTTON_MOTION_MASK
                | gdk::EventMask::KEY_PRESS_MASK
                | gdk::EventMask::KEY_RELEASE_MASK
                | gdk::EventMask::ENTER_NOTIFY_MASK
                | gdk::EventMask::LEAVE_NOTIFY_MASK
                | gdk::EventMask::SCROLL_MASK,
        );

        //let self_ = obj.downcast_ref::<MyWidget>().unwrap();

        //let mut p = self.priv_.borrow_mut();
        //p.nb_pixs_columns = 32;
        //p.nb_pixs_rows =  32;

        //p.sprite = Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap();
        //p.sprite.fill(0x00000000);

        let widget = self.get_instance();
        widget.set_size_request(MINIMUM_WIDTH, MINIMUM_HEIGHT);
        widget.set_can_focus(true);

        widget.connect_key_press_event(
            {
                move |s, e| 
                {
                    //let widget_priv = EditAreaPrivate::from_instance(s); Ok
                    //let p = widget_priv.priv_.borrow(); Ok
                    if e.get_keyval()==keys::constants::a {
                        let w = s.get_window().unwrap();
                        if let Some(display) = gdk::Display::get_default() {
                            w.set_cursor(Some(&Cursor::new_for_display(
                                &display,
                                gdk::CursorType::CoffeeMug,
                            )));
                        }                
                        println!("Key Press Edit Area1...");
                    }
                    //--
                    Inhibit(false)
    
                }
            }
        );

        // self.get_instance().connect_key_release_event(
        //     {
        //         |s, e| 
        //         {
        //             //let widget_priv = EditAreaPrivate::from_instance(s); Ok
        //             //let p = widget_priv.priv_.borrow(); Ok
            
        //             let w = s.get_window().unwrap();
        //             if let Some(display) = gdk::Display::get_default() {
        //                 w.set_cursor(Some(&Cursor::new_for_display(
        //                     &display,
        //                     gdk::CursorType::Arrow,
        //                 )));
        //             }                
        //             println!("Key Release Edit Area1...");
        //             //--
        //             Inhibit(false)
    
        //         }
        //     }
        // );

        //p.select_handles.push(RHandle::new(0,0,0,0,p.start_point_x,p.start_point_y));
        //let h1 = p.select_handles.get_mut(0).unwrap();
        //h1.normalize();
        //p.select_handles.push(RHandle::new(0,0,0,0));
        //p.select_handles.push(RHandle::new(0,0,0,0));
        //p.select_handles.push(RHandle::new(0,0,0,0));

        // let mut mask = widget.get_events();
        // mask |= gdk::EventMask::BUTTON_PRESS_MASK;
        // widget.set_events(mask);

    }
}

impl EditAreaPrivate {}

impl DrawingAreaImpl for EditAreaPrivate {}

impl WidgetImpl for EditAreaPrivate {


    fn draw(&self, _widget: &gtk::Widget, cr: &cairo::Context) -> Inhibit {
        //--
        let mut p = self.priv_.borrow_mut();
        (p.draw) (&mut p, _widget, cr);
        //--
        Inhibit(false)

    }

    fn button_press_event(&self, _widget: &gtk::Widget, event: &gdk::EventButton) -> Inhibit {
        //--
        //_widget.set_can_focus(true);
        _widget.grab_focus();
        let mut p = self.priv_.borrow_mut();
        if event.get_button()==2 {
            if let Some((x, y)) = event.get_coords() {
                p.start_x = x as i32;
                p.start_y = y as i32;
                p.start_origin_x = p.origin_x;
                p.start_origin_y = p.origin_y;    
            }
        }else{
            (p.button_press_event) (&mut p, _widget, event);
        }
        //let widget1 = self.get_instance();
        //let x:u32 = 0;
        //let y:u32 = 0;
        //widget1.emit("left-clicked",&[]);
        //widget1.emit("color-changed", &[&x,&y]);

        //--
        Inhibit(false)

    }

    fn button_release_event(&self, _widget: &gtk::Widget, event: &gdk::EventButton) -> Inhibit {
        //--
        let mut p = self.priv_.borrow_mut();
        (p.button_release_event) (&mut p, _widget, event);
        
        //--
        Inhibit(false)

    }

    fn motion_notify_event(&self, _widget: &gtk::Widget, event: &gdk::EventMotion) -> Inhibit {
        //--
        let mut p = self.priv_.borrow_mut();
        let state: gdk::ModifierType = event.get_state();
        if state.contains(gdk::ModifierType::BUTTON2_MASK) {
            if let Some((x, y)) = event.get_coords() {
                let dx = (x as i32) - p.start_x;
                let dy = (y as i32) - p.start_y;
                p.origin_x = p.start_origin_x + dx;
                p.origin_y = p.start_origin_y + dy;    
                _widget.queue_draw();
            }
        }else{
            (p.motion_notify_event) (&mut p, _widget, event);
        }
        //--
        Inhibit(false)
       
    }

    fn scroll_event(&self, _widget: &gtk::Widget, event: &gdk::EventScroll)-> Inhibit {
        //--
        let mut p = self.priv_.borrow_mut();
        if event.get_direction()==gdk::ScrollDirection::Up {
            if p.m_scale<5.0f32 {
                p.m_scale += 0.05f32;
    
            }
        }else{
            if p.m_scale>0.5f32 {
                p.m_scale -= 0.05f32;
            }
        }

        _widget.queue_draw();
   
        //--
        Inhibit(false)

    }
    
}

glib_wrapper! {
    pub struct EditArea(
        Object<subclass::simple::InstanceStruct<EditAreaPrivate>,
        subclass::simple::ClassStruct<EditAreaPrivate>,
        EditAreaClass>)
        @extends gtk::DrawingArea, gtk::Widget;

    match fn {
        get_type => || EditAreaPrivate::get_type().to_glib(),
    }
}

impl EditArea {

    pub fn new() -> Self {
        glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create MyWidget")
            .downcast()
            .expect("Created MyWidget is of wrong type")

    }

    pub fn get_sprite(&self) -> Rc<Pixbuf> {
        //--
        let widget_priv = EditAreaPrivate::from_instance(self);
        let p = widget_priv.priv_.borrow();
        p.sprite.clone()
    }

    pub fn set_sprite(&self, spr1: Rc<Pixbuf>) {
        //--
        let widget_priv = EditAreaPrivate::from_instance(self);
        let widget = widget_priv.get_instance();
        let mut p = widget_priv.priv_.borrow_mut();
        p.sprite = spr1.clone();
        p.nb_pixs_columns = spr1.get_width();
        p.nb_pixs_rows = spr1.get_height();
        p.sprite_bak = Pixbuf::new(Colorspace::Rgb, true, 8, p.nb_pixs_columns, p.nb_pixs_rows).unwrap();
        //p.sprite_bak.fill(p.background_color);
        p.sprite_copy = Pixbuf::new(Colorspace::Rgb, true, 8, p.nb_pixs_columns, p.nb_pixs_rows).unwrap();
        p.copy_rect.empty();

        widget.queue_draw();
    }


    pub fn set_colors(&self, foreground_color: u32, background_color: u32){
        //--
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.foreground_color = foreground_color;
        p.background_color = background_color;

    }

    pub fn set_select_mode(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.mode = EditMode::SELECT;
        p.init_select_mode();
        p.draw = EditAreaDatas::draw_select_mode;
        p.button_press_event = EditAreaDatas::button_press_event_select_mode;
        p.button_release_event = EditAreaDatas::button_release_event_select_mode;
        p.motion_notify_event = EditAreaDatas::motion_notify_event_select_mode;
        let widget = widget_priv.get_instance();
        widget.queue_draw();
        println!("EditArea Select Mode");
    }

    pub fn set_pencil_mode(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.mode = EditMode::PENCIL;
        p.init_select_mode();
        p.draw = EditAreaDatas::draw_pencil_mode;
        p.button_press_event = EditAreaDatas::button_press_event_pencil_mode;
        p.button_release_event = EditAreaDatas::button_release_event_pencil_mode;
        p.motion_notify_event = EditAreaDatas::motion_notify_event_pencil_mode;
        let widget = widget_priv.get_instance();
        widget.queue_draw();
        println!("EditArea Pencil Mode");
    }

    pub fn set_rectangle_mode(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.mode = EditMode::RECTANGLE;
        p.init_select_mode();
        p.init_rectangle_mode();
        p.draw = EditAreaDatas::draw_rectangle_mode;
        p.button_press_event = EditAreaDatas::button_press_event_rectangle_mode;
        p.button_release_event = EditAreaDatas::button_release_event_rectangle_mode;
        p.motion_notify_event = EditAreaDatas::motion_notify_event_rectangle_mode;
        let widget = widget_priv.get_instance();
        widget.queue_draw();
        println!("EditArea Rectangle Mode");
    }

    pub fn set_ellipse_mode(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.mode = EditMode::ELLIPSE;
        p.init_select_mode();
        p.init_ellipse_mode();
        p.draw = EditAreaDatas::draw_ellipse_mode;
        p.button_press_event = EditAreaDatas::button_press_event_ellipse_mode;
        p.button_release_event = EditAreaDatas::button_release_event_ellipse_mode;
        p.motion_notify_event = EditAreaDatas::motion_notify_event_ellipse_mode;
        let widget = widget_priv.get_instance();
        widget.queue_draw();
        println!("EditArea Rectangle Mode");
    }

    pub fn set_fill_mode(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.mode = EditMode::FILL;
        p.init_select_mode();
        p.init_fill_mode();
        p.draw = EditAreaDatas::draw_fill_mode;
        p.button_press_event = EditAreaDatas::button_press_event_fill_mode;
        p.button_release_event = EditAreaDatas::button_release_event_fill_mode;
        p.motion_notify_event = EditAreaDatas::motion_notify_event_fill_mode;
        let widget = widget_priv.get_instance();
        widget.queue_draw();
        println!("EditArea Fill Mode");
    }

    pub fn edit_copy(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        if p.mode==EditMode::SELECT {
            
            if p.select_rect.is_empty()==false {

                //-- Update Clipboard
                let clip_image = Pixbuf::new(Colorspace::Rgb, true, 8, p.select_rect.width(), p.select_rect.height()).unwrap();
                p.sprite.copy_area( p.select_rect.left, p.select_rect.top,
                    p.select_rect.width(), p.select_rect.height(), &clip_image,0,0);
                let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
                clipboard.set_image(&clip_image);

                //p.copy_select();
                p.sprite.copy_area( p.select_rect.left, p.select_rect.top,
                    p.select_rect.width(), p.select_rect.height(), &p.sprite_copy,0,0);
                p.copy_rect.left = p.select_rect.left;
                p.copy_rect.right = p.select_rect.right;
                p.copy_rect.top = p.select_rect.top;
                p.copy_rect.bottom = p.select_rect.bottom;
                p.select_rect.empty();
    
                let widget = widget_priv.get_instance();
                widget.queue_draw();

            }
        }
    }

    pub fn edit_paste(&self){
        {
            
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            if clipboard.wait_is_image_available() {
                
                self.set_select_mode();

                clipboard.request_image(
                    {
                        let self1 = self.clone();
                        move |_, img| {
                            let wi = img.get_width();
                            let hi = img.get_height();
                            let widget_priv1 = EditAreaPrivate::from_instance(&self1);
                            let mut p1 = widget_priv1.priv_.borrow_mut();
                            let w = if (wi<=p1.nb_pixs_columns){
                                        wi
                                    }else{
                                        p1.nb_pixs_columns
                                    };
                            let h = if (hi<=p1.nb_pixs_rows){
                                        hi
                                    }else{
                                        p1.nb_pixs_rows
                                    };
                            p1.copy_rect.left = 0;
                            p1.copy_rect.top  = 0;
                            p1.copy_rect.right = w-1;
                            p1.copy_rect.bottom = h-1;
                            p1.sprite_copy = p1.sprite.copy().unwrap();
                            img.copy_area( p1.copy_rect.left, p1.copy_rect.top,
                                p1.copy_rect.width(), p1.copy_rect.height(), &p1.sprite_copy,0,0);

                            p1.select_rect.left = p1.copy_rect.left;
                            p1.select_rect.right = p1.copy_rect.right;
                            p1.select_rect.top = p1.copy_rect.top;
                            p1.select_rect.bottom = p1.copy_rect.bottom;
                            p1.select_rect.mode = 2;
                            p1.backup_sprite();
                            p1.sprite_copy.copy_area( 0, 0, w, h,
                                                &p1.sprite,p1.select_rect.left,p1.select_rect.top);
                            println!("Get Clipboard {:?}", img);
                        }
                    }
                );
                
            }
        }

        let widget_priv = EditAreaPrivate::from_instance(self);
        //let mut p = widget_priv.priv_.borrow_mut();
        //p.paste_select();
        let widget = widget_priv.get_instance();        
        widget.queue_draw();
        if let Err(why) = widget.emit("edit-changed",&[]) {
            println!("{:?}", why);
        }

    }

    pub fn edit_cut(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        if p.mode==EditMode::SELECT {

            p.cut_select();

            let widget = widget_priv.get_instance();
            widget.queue_draw();
            if let Err(why) = widget.emit("edit-changed",&[]) {
                println!("{:?}", why);
            }
        }
        
    }

    pub fn edit_undo(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let undo_mode : UndoMode;

        {
            let mut p = widget_priv.priv_.borrow_mut();
            undo_mode = match p.undo_mode{
                UndoMode::PENCIL|UndoMode::RECTANGLE|UndoMode::ELLIPSE|
                UndoMode::FILL => {
                    let sprite_redo = Pixbuf::new(Colorspace::Rgb, true, 8, p.nb_pixs_columns, p.nb_pixs_rows).unwrap();
                    p.sprite.copy_area( 0, 0, p.nb_pixs_columns, p.nb_pixs_rows, &sprite_redo, 0, 0);
                    p.sprite_bak.copy_area( 0, 0, p.nb_pixs_columns, p.nb_pixs_rows, &p.sprite, 0, 0);
                    sprite_redo.copy_area( 0, 0, p.nb_pixs_columns, p.nb_pixs_rows, &p.sprite_bak, 0, 0);
                    p.undo_mode
                }
                UndoMode::FLIP_HORIZONTALY=>{
                    self.flip_horizontaly();
                    UndoMode::FLIP_HORIZONTALY
                }
                UndoMode::FLIP_VERTICALY=>{
                    self.flip_verticaly();
                    UndoMode::FLIP_VERTICALY
                }
                UndoMode::SWING_LEFT => {
                    UndoMode::SWING_LEFT
                }
                UndoMode::SWING_RIGHT => {
                    UndoMode::SWING_RIGHT
                }
                _ => {
                    UndoMode::NONE
                }
            }
        }

        if undo_mode==UndoMode::SWING_LEFT {
            self.swing_right();

        }else if undo_mode==UndoMode::SWING_RIGHT{
            self.swing_left();
        }
    
        let widget = widget_priv.get_instance();
        widget.queue_draw();
    
    }

    pub fn flip_horizontaly(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.backup_sprite();
        p.do_flip_horizontaly();
        let widget = widget_priv.get_instance();
        widget.queue_draw();

    }

    pub fn flip_verticaly(&self){
        let widget_priv = EditAreaPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.backup_sprite();
        p.do_flip_verticaly();
        let widget = widget_priv.get_instance();
        widget.queue_draw();

    }

    pub fn swing_left(&self)
    {
        let widget_priv = EditAreaPrivate::from_instance(self);
        {
            let mut p = widget_priv.priv_.borrow_mut();
            
            let w = p.sprite.get_width();
            let h = p.sprite.get_height();

            //-- Agrandir avant de faire le backup
            if w!=h { 
                let d_max = if h>w{
                                h
                            }else{
                                w
                            };
                p.sprite_bak = Pixbuf::new(Colorspace::Rgb, true, 8, d_max, d_max).unwrap();
                p.sprite_copy = Pixbuf::new(Colorspace::Rgb, true, 8, d_max, d_max).unwrap();
            }
            p.sprite.copy_area( 0, 0, w, h, &p.sprite_bak, 0, 0);

            //--
            let sprite_des = Pixbuf::new(Colorspace::Rgb, true, 8, h, w).unwrap();

            p.undo_mode = UndoMode::SWING_LEFT;

            unsafe{

                let row_stride_des = sprite_des.get_rowstride();
                let n_channels_des = sprite_des.get_n_channels();
                let pixels_des = sprite_des.get_pixels();

                let row_stride_src = p.sprite.get_rowstride();
                let n_channels_src = p.sprite.get_n_channels();
                let pixels_src = p.sprite.get_pixels();

                let mut ip_src: usize;
                let mut ip_des: usize;

                let offset_h = w -1;
                for x in 0..w {
                    for y in 0..h {
                        ip_src = ( x * n_channels_src + y * row_stride_src) as usize;
                        ip_des = ( y * n_channels_des + (offset_h-x) * row_stride_des) as usize;
                        pixels_des[ip_des] = pixels_src[ip_src];
                        pixels_des[ip_des+1] = pixels_src[ip_src+1];
                        pixels_des[ip_des+2] = pixels_src[ip_src+2];
                        pixels_des[ip_des+3] = pixels_src[ip_src+3];

                    }
                }
            }
            p.sprite = Rc::new(sprite_des);
            let w = p.sprite.get_width();
            let h = p.sprite.get_height();
            p.nb_pixs_columns = w;
            p.nb_pixs_rows = h;
    
        }
        let widget = widget_priv.get_instance();
        if let Err(why) = widget.emit("sprite-transform",&[]) {
            println!("{:?}", why);
        }
        widget.queue_draw();

    }

    pub fn swing_right(&self)
    {
        let widget_priv = EditAreaPrivate::from_instance(self);
        {
            let mut p = widget_priv.priv_.borrow_mut();

            let w = p.sprite.get_width();
            let h = p.sprite.get_height();

            //-- Agrandir avant de faire le backup
            if w!=h {
                let d_max = if h>w{
                    h
                }else{
                    w
                };
                p.sprite_bak = Pixbuf::new(Colorspace::Rgb, true, 8, d_max, d_max).unwrap();
                p.sprite_copy = Pixbuf::new(Colorspace::Rgb, true, 8, d_max, d_max).unwrap();
            }
            p.sprite.copy_area( 0, 0, w, h, &p.sprite_bak, 0, 0);

            //--
            let sprite_des = Pixbuf::new(Colorspace::Rgb, true, 8, h, w).unwrap();

            p.undo_mode = UndoMode::SWING_RIGHT;

            unsafe{

                let row_stride_des = sprite_des.get_rowstride();
                let n_channels_des = sprite_des.get_n_channels();
                let pixels_des = sprite_des.get_pixels();

                let row_stride_src = p.sprite.get_rowstride();
                let n_channels_src = p.sprite.get_n_channels();
                let pixels_src = p.sprite.get_pixels();

                let mut ip_src: usize;
                let mut ip_des: usize;

                let offset_w = h - 1;
                for x in 0..w {
                    for y in 0..h {
                        ip_src = ( x * n_channels_src + y * row_stride_src) as usize;
                        ip_des = ( (offset_w - y) * n_channels_des + x * row_stride_des) as usize;
                        pixels_des[ip_des] = pixels_src[ip_src];
                        pixels_des[ip_des+1] = pixels_src[ip_src+1];
                        pixels_des[ip_des+2] = pixels_src[ip_src+2];
                        pixels_des[ip_des+3] = pixels_src[ip_src+3];

                    }
                }
            }
            p.sprite = Rc::new(sprite_des);
            let w = p.sprite.get_width();
            let h = p.sprite.get_height();
            p.nb_pixs_columns = w;
            p.nb_pixs_rows = h;

        }
        let widget = widget_priv.get_instance();
        if let Err(why) = widget.emit("sprite-transform",&[]) {
            println!("{:?}", why);
        }
        widget.queue_draw();

    }

}

pub trait EditAreaExt {
    /// Connect to signal left-clicked
    fn connect_edit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId;
    fn connect_sprite_transform<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId;
    fn connect_color_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;
    fn connect_pick_foreground_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;
    fn connect_pick_background_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;

}

impl EditAreaExt for EditArea {

    fn connect_edit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn edit_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkWidget,
            f: glib_sys::gpointer,
        ) where
            P: IsA<gtk::Widget>,
        {
            let f: &F = &*(f as *const F);
            f(&gtk::Widget::from_glib_none(this).unsafe_cast())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"edit-changed\0".as_ptr() as *const _,
                Some(transmute(edit_changed_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    fn connect_sprite_transform<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn edit_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkWidget,
            f: glib_sys::gpointer,
        ) where
            P: IsA<gtk::Widget>,
        {
            let f: &F = &*(f as *const F);
            f(&gtk::Widget::from_glib_none(this).unsafe_cast())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"sprite-transform\0".as_ptr() as *const _,
                Some(transmute(edit_changed_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    fn connect_color_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn color_changed_trampoline<P, F: Fn(&P, u32, u32) + 'static>(
            this: *mut gtk_sys::GtkWidget,
            fore_color: u32,
            back_color: u32,
            f: glib_sys::gpointer,
        ) where
            P: IsA<gtk::Widget>,
        {
            let f: &F = &*(f as *const F);
            f(&gtk::Widget::from_glib_none(this).unsafe_cast(), fore_color, back_color)
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"color-changed\0".as_ptr() as *const _,
                Some(transmute(color_changed_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    fn connect_pick_foreground_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn pick_foreground_color_trampoline<P, F: Fn(&P, u32) + 'static>(
            this: *mut gtk_sys::GtkWidget,
            idcol: u32,
            f: glib_sys::gpointer,
        ) where
            P: IsA<gtk::Widget>,
        {
            let f: &F = &*(f as *const F);
            f(&gtk::Widget::from_glib_none(this).unsafe_cast(), idcol)
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"pick-foreground-color\0".as_ptr() as *const _,
                Some(transmute(pick_foreground_color_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    fn connect_pick_background_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn pick_background_color_trampoline<P, F: Fn(&P, u32) + 'static>(
            this: *mut gtk_sys::GtkWidget,
            idcol: u32,
            f: glib_sys::gpointer,
        ) where
            P: IsA<gtk::Widget>,
        {
            let f: &F = &*(f as *const F);
            f(&gtk::Widget::from_glib_none(this).unsafe_cast(), idcol)
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"pick-background-color\0".as_ptr() as *const _,
                Some(transmute(pick_background_color_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }

}
