//! 
//!
//! 
//! 
//! 

// use std::mem::transmute;
// use gio::prelude::*;
// use gtk::prelude::*;

// use gio::subclass::prelude::*;
// //use gio::ApplicationFlags;
// use glib::Type;
// use glib::subclass;
// use glib::translate::*;
// use gtk::subclass::prelude::*;

// use once_cell::unsync::OnceCell;
// use std::cell::{Cell, RefCell};
// use gdk_pixbuf::*;

// use cairo::{Context, ImageSurface, Format};

// use gdk::prelude::GdkContextExt;
// use std::rc::Rc;



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

use gdk::prelude::GdkContextExt;
use gdk::{RGBA, WindowExt, Cursor, Display, keys};


const MINIMUM_WIDTH: i32 = 66;
const MINIMUM_HEIGHT: i32 = 64*8;

#[derive(Debug)]
struct SpritesBarDatas {
    nb_columns: i32,
    nb_rows: i32,
    cell_size: i32,
    tbl_sprites: [Rc<Pixbuf>;8],
    tbl_file_names:[String;8],
    i_cur_sprite: u32,

}

impl SpritesBarDatas {
    fn new() -> Self {
        Self {
            nb_columns: 1,
            nb_rows: 8,
            cell_size: 64,
            tbl_sprites:[
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
                            Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap())
                        ],
            tbl_file_names:[
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from("")
                            ],
            i_cur_sprite: 0,
        }
    }
}

// This is the private part of our `PaletteDatas` object.
// Its where state and widgets are stored when they don't
// need to be publicly accesible.
#[derive(Debug)]
pub struct SpritesBarPrivate {
    priv_: RefCell<SpritesBarDatas>,
    counter: Cell<u64>,

}

impl ObjectSubclass for SpritesBarPrivate {
    const NAME: &'static str = "SpritesBar";
    type ParentType = gtk::DrawingArea;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.add_signal(
            "choose-color",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32],
            Type::Unit,
        );
        klass.add_signal(
            "sprite-changed",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32],
            Type::Unit,
        );
    }

    fn new() -> Self {
        Self {
            priv_: RefCell::new(SpritesBarDatas::new()),
            counter: Cell::new(0),

        }
    }
}

impl ObjectImpl for SpritesBarPrivate {
    glib_object_impl!();

    // Here we are overriding the glib::Objcet::contructed
    // method. Its what gets called when we create our Object
    // and where we can initialize things.
    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
        
        obj.downcast_ref::<gtk::Widget>().unwrap().add_events(
            gdk::EventMask::BUTTON_PRESS_MASK
                | gdk::EventMask::BUTTON_RELEASE_MASK
                | gdk::EventMask::BUTTON_MOTION_MASK,
        );

        //let self_ = obj.downcast_ref::<MyWidget>().unwrap();

        let mut p = self.priv_.borrow_mut();
        p.nb_columns = 1;
        p.nb_rows =  8;

        for i in 0..8 {
            p.tbl_sprites[i].fill(0x00000000);
        }

        self.get_instance()
            .set_size_request(MINIMUM_WIDTH, MINIMUM_HEIGHT);

        //-- 
        // let widget = self.get_instance();
        // let mut mask = widget.get_events();
        // mask |= gdk::EventMask::BUTTON_PRESS_MASK;
        // widget.set_events(mask);

    }
}

impl SpritesBarPrivate {}

impl DrawingAreaImpl for SpritesBarPrivate {}

impl WidgetImpl for SpritesBarPrivate {

    fn draw(&self, _widget: &gtk::Widget, cr: &cairo::Context) -> Inhibit {
        //--
        let w = _widget.get_allocation().width - 1;
        let h = _widget.get_allocation().height - 1;
        let p = self.priv_.borrow_mut();

        let nb_cols = p.nb_columns;
        let nb_rows = p.nb_rows;
        let cell_size= p.cell_size;

        cr.set_line_width (0.5);

        let mut x: i32;
        let mut y: i32;

        for row in 0..=nb_rows {
            y = row * cell_size;
            for col in 0..=nb_cols {
                x = col * cell_size;
                cr.move_to(x as f64, y as f64);
                cr.line_to((x+cell_size) as f64, y as f64);
                //cr.line_to((x+cell_size) as f64, (y+cell_size) as f64);
                //cr.line_to(x as f64, (y+cell_size) as f64);
                //cr.line_to(x as f64, y as f64);
            }
        }

        y = 0;
        x = 0;
        cr.move_to(x as f64, y as f64);
        y = nb_rows * cell_size;
        cr.line_to(x as f64, y as f64);
        y = 0;
        x = cell_size+1;
        cr.move_to(x as f64, y as f64);
        y = nb_rows * cell_size;
        cr.line_to(x as f64, y as f64);

        cr.stroke();

        //-- Draw sprite

        let mut y : f64;
        let mut x : f64;
        for i in 0..8 {
            let h = p.tbl_sprites[i as usize].get_height();
            let w = p.tbl_sprites[i as usize].get_width();
            x = ((p.cell_size-w) as f64)/ 2.0; 
            y = (i*p.cell_size + (p.cell_size-h)/2) as f64;
            cr.set_source_pixbuf(&(p.tbl_sprites[i as usize]), x, y);
            cr.paint();
        }

        //-- Draw Select Frame
        cr.set_line_width(1.0);
        cr.set_source_rgb(1.0, 0.0, 0.0);
        let corner_size = 10f64;
        let dbl_size =  p.cell_size as f64;
        let left = 1f64;
        let top  = dbl_size * (p.i_cur_sprite as f64) + 1.0;
        let right = left + dbl_size;
        let bottom = top + dbl_size;
        //-- Top Left corner
        cr.move_to(left + corner_size, top);
        cr.line_to(left, top);
        cr.line_to(left, top + corner_size);
        //-- Top Right corner
        cr.move_to(right - corner_size, top);
        cr.line_to(right, top);
        cr.line_to(right, top + corner_size);
        //-- Bottom Left corner
        cr.move_to(left + corner_size, bottom);
        cr.line_to(left, bottom);
        cr.line_to(left, bottom - corner_size);
        //-- Bottom Right corner
        cr.move_to(right - corner_size, bottom);
        cr.line_to(right, bottom);
        cr.line_to(right, bottom - corner_size);

        cr.stroke();

        //--
        Inhibit(false)

    }

    fn button_press_event(&self, _widget: &gtk::Widget, event: &gdk::EventButton) -> Inhibit {

        //let state: gdk::ModifierType = event.get_state();
        let widget1 = self.get_instance();

        let mut f_select_change : bool = false;
        let mut i_cur_sprite: u32 = 0;
        if let Some((x, y)) = event.get_coords() {
            let mut p = self.priv_.borrow_mut();
            let cell_size = p.cell_size as f64;
            if x < cell_size {
                let i = (y / cell_size) as u32;
                if (i<(p.nb_rows as u32)) && (i!=p.i_cur_sprite) {
                    p.i_cur_sprite = i;
                    widget1.queue_draw();
                    println!("Select i_sprite={}", p.i_cur_sprite);
                    f_select_change = true;
                    i_cur_sprite = p.i_cur_sprite;
                }
            }
        }

        if f_select_change { // Emit Here to free borrow mut p
            widget1.emit("sprite-changed",&[&i_cur_sprite]);
        }

        Inhibit(false)

    }

    fn button_release_event(&self, _widget: &gtk::Widget, event: &gdk::EventButton) -> Inhibit {
        //--
        if let Some((x, y)) = event.get_coords() {
            println!("Release x={} y={}", x, y);
        }

        //--
        Inhibit(false)
    }

    fn motion_notify_event(&self, _widget: &gtk::Widget, event: &gdk::EventMotion) -> Inhibit {
        //---
        //let state: gdk::ModifierType = event.get_state();

        //--
        Inhibit(false)
       
    }

}

glib_wrapper! {
    pub struct SpritesBar(
        Object<subclass::simple::InstanceStruct<SpritesBarPrivate>,
        subclass::simple::ClassStruct<SpritesBarPrivate>,
        SpritesBarPrivateClass>)
        @extends gtk::DrawingArea, gtk::Widget;

    match fn {
        get_type => || SpritesBarPrivate::get_type().to_glib(),
    }

}

impl SpritesBar {

    pub fn new() -> Self {
        glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create Palette Widget")
            .downcast()
            .expect("Created MyWidget is of wrong type")
    }

    pub fn get_sprite(&self) -> Rc<Pixbuf> {
        //--
        let widget_priv = SpritesBarPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        let i = p.i_cur_sprite as usize;
        p.tbl_sprites[i].clone()

    }

    pub fn set_sprite(&self, spr1: Rc<Pixbuf>) {
        //--
        let widget_priv = SpritesBarPrivate::from_instance(self);
        //let widget = widget_priv.get_instance();
        let mut p = widget_priv.priv_.borrow_mut();
        let i = p.i_cur_sprite as usize;
        p.tbl_sprites[i] = spr1.clone();
        self.queue_draw();
    }

    pub fn new_sprite(&self, w: i32, h:i32) {
        //--
        let widget_priv = SpritesBarPrivate::from_instance(self);
        let i_cur_sprite;
        {
            let mut p = widget_priv.priv_.borrow_mut();
            let i = p.i_cur_sprite as usize;
            p.tbl_sprites[i] = Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, w, h).unwrap());
            p.tbl_sprites[i].fill(0x00000000); 
            p.tbl_file_names[i] = String::from("");
            i_cur_sprite = p.i_cur_sprite as u32;
        } // This Block release mut borrow before emitting signal

        self.queue_draw();
        self.emit("sprite-changed",&[&i_cur_sprite]);

    }

    pub fn load_sprite(&self, full_path_name : &str){
        //--
        let widget_priv = SpritesBarPrivate::from_instance(self);
        let i_cur_sprite;
        {
            let mut p = widget_priv.priv_.borrow_mut();
            let i = p.i_cur_sprite as usize;
            p.tbl_sprites[i] = Rc::new(Pixbuf::from_file(full_path_name).unwrap());
            p.tbl_file_names[i] = String::from(full_path_name);
            i_cur_sprite = p.i_cur_sprite;
        }
        self.queue_draw();
        self.emit("sprite-changed",&[&i_cur_sprite]);
        
    }

    pub fn save_sprite(&self){
        //--
        let widget_priv = SpritesBarPrivate::from_instance(self);
        let p = widget_priv.priv_.borrow();
        let i = p.i_cur_sprite as usize;
        if p.tbl_file_names[i]!="" {
            let full_path_name: &str = &p.tbl_file_names[i];
            p.tbl_sprites[i].savev(&full_path_name,"png",&[("","")]);
            println!("File Name {}", full_path_name);
        }
    }

    pub fn get_file_name(&self)->String {
        let widget_priv = SpritesBarPrivate::from_instance(self);
        let p = widget_priv.priv_.borrow();
        let i = p.i_cur_sprite as usize;
        let name = String::from(&p.tbl_file_names[i]);
        name
    }

    pub fn save_as_sprite(&self, full_path_name : &str){
        //--
        let widget_priv = SpritesBarPrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        let i = p.i_cur_sprite as usize;
        p.tbl_file_names[i] = String::new();
        p.tbl_file_names[i].push_str(full_path_name);
        let i = p.i_cur_sprite as usize;
        p.tbl_sprites[i].savev(full_path_name,"png",&[("","")]);

    }

    pub fn fresh_display(&self){
        //let widget_priv = SpritesBarPrivate::from_instance(self);
        //let mut p = widget_priv.priv_.borrow_mut();
        self.queue_draw();
        println!("SpritesBar => fresh_display");

    }

}

pub trait SpritesBarExt {
    /// Connect to signal note-pressed
    fn connect_choose_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;
    fn connect_sprite_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;

}

impl SpritesBarExt for SpritesBar {

    fn connect_sprite_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn sprite_changed_trampoline<P, F: Fn(&P, u32) + 'static>(
            this: *mut gtk_sys::GtkWidget,
            i_sel_sprite: u32,
            f: glib_sys::gpointer,
        ) where
            P: IsA<gtk::Widget>,
        {
            let f: &F = &*(f as *const F);
            f(&gtk::Widget::from_glib_none(this).unsafe_cast(), i_sel_sprite)
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"sprite-changed\0".as_ptr() as *const _,
                Some(transmute(sprite_changed_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    fn connect_choose_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        unsafe extern "C" fn choose_color_trampoline<P, F: Fn(&P, u32) + 'static>(
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
                b"choose-color\0".as_ptr() as *const _,
                Some(transmute(choose_color_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }


}

