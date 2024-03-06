//! 
//!
//! 
//! 
//! 

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, LineWriter};
use std::path::Path;


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


use gtk::{ColorChooserDialog,ResponseType};


// mod rgb_utils; Sinon Erreur à l a compilation
use crate::rgb_utils::{get_rgba};

const MINIMUM_WIDTH: i32 = 256;
const MINIMUM_HEIGHT: i32 = 48;

#[derive(Debug)]
struct PaletteDatas {
    nb_columns: i32,
    nb_rows: i32,
    cell_size: i32,
    colors: Vec<u32>,
    foreground_color: u32,
    background_color: u32,
}

impl PaletteDatas {
    fn new() -> Self {
        Self {
            nb_columns: 14,
            nb_rows: 2,
            cell_size: 12,
            colors: Vec::new(),
            foreground_color: 0x0000FFFF,
            background_color: 0x00,
        }
    }
}

// This is the private part of our `PaletteDatas` object.
// Its where state and widgets are stored when they don't
// need to be publicly accesible.
#[derive(Debug)]
pub struct PalettePrivate {
    priv_: RefCell<PaletteDatas>,
    counter: Cell<u64>,

}

impl ObjectSubclass for PalettePrivate {
    const NAME: &'static str = "Palette";
    type ParentType = gtk::DrawingArea;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.add_signal(
            "color-changed",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32,Type::U32],
            Type::Unit,
        );
        klass.add_signal(
            "choose-color",
            glib::SignalFlags::RUN_LAST|glib::SignalFlags::NO_RECURSE|glib::SignalFlags::NO_HOOKS,
            &[Type::U32],
            Type::Unit,
        );
    }

    fn new() -> Self {
        Self {
            priv_: RefCell::new(PaletteDatas::new()),
            counter: Cell::new(0),

        }
    }
}

impl ObjectImpl for PalettePrivate {
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
        p.nb_columns = 32;
        p.nb_rows =  2;

        p.foreground_color = 0x0000FFFF;
        p.background_color = 0x00;

        for i in 0..( p.nb_columns*p.nb_rows) {
            p.colors.push(0xE0E0E0FF);
        }
        
        let mut i: usize = 0;
        p.colors[i] = 0x00000000;
        i += 1;
        p.colors[i] = 0x000000FF;
        i += 1;
        p.colors[i] = 0xFFFFFFFF;
        i += 1;
        p.colors[i] = 0x808080FF;
        i += 1;
        p.colors[i] = 0xC0C0C0FF;
        i += 1;
        p.colors[i] = 0x800000FF;
        i += 1;
        p.colors[i] = 0xFF0000FF;
        i += 1;
        p.colors[i] = 0x808000FF;
        i += 1;
        p.colors[i] = 0xFFFF00FF;
        i += 1;
        p.colors[i] = 0x008000FF;
        i += 1;
        p.colors[i] = 0x00FF00FF;
        i += 1;
        p.colors[i] = 0x008080FF;
        i += 1;
        p.colors[i] = 0x00FFFFFF;
        i += 1;
        p.colors[i] = 0x000080FF;
        i += 1;
        p.colors[i] = 0x800080FF;
        i += 1;
        p.colors[i] = 0xFF00FFFF;

        self.get_instance()
            .set_size_request(MINIMUM_WIDTH, MINIMUM_HEIGHT);

        //-- 
        // let widget = self.get_instance();
        // let mut mask = widget.get_events();
        // mask |= gdk::EventMask::BUTTON_PRESS_MASK;
        // widget.set_events(mask);

    }
}

impl PalettePrivate {

    fn draw_colors(&self, cr: &cairo::Context){
        //---------------------------------------------------------
        //--
        let widget1 = self.get_instance();
        let w = widget1.get_allocation().width - 1;
        let h = widget1.get_allocation().height - 1;

        let p = self.priv_.borrow();
        let nb_cols = p.nb_columns;
        let nb_rows = p.nb_rows;
        let cell_size= p.cell_size;

        let mut icol: usize;
        let d = (cell_size-2) as f64;
        for row in 0..nb_rows {
            let y = (row * cell_size + 4) as f64;
            for col in 0..nb_cols {
                icol = (col + row * p.nb_columns) as usize;
                let c = p.colors[icol];
                let (r,g,b,a) = get_rgba(c);
                cr.set_source_rgba((r as f64)/255.0,(g as f64)/255.0,(b as f64)/255.0,(a as f64)/255.0);
                let x = ((col+2) * cell_size + 4) as f64;
                cr.rectangle(x, y, d, d);
                cr.fill();
            }
        }

        //--
        let (r,g,b,a) = get_rgba(p.background_color as u32);
        cr.set_source_rgba((r as f64)/255.0,(g as f64)/255.0,(b as f64)/255.0,(a as f64)/255.0);
        let d = (p.cell_size*2) as f64;
        cr.rectangle(1.0, 1.0, d, d);
        cr.fill();

        let (r,g,b,a) = get_rgba(p.foreground_color as u32);
        cr.set_source_rgba((r as f64)/255.0,(g as f64)/255.0,(b as f64)/255.0,(a as f64)/255.0);
        let d = ((p.cell_size as f64)*1.4) as f64;
        cr.rectangle(1.0, 1.0, d, d);
        cr.fill();

        //-- Draw sprite
        //let x = p.cell_size * (p.nb_pixs_columns+2);
        //cr.set_source_pixbuf(&p.sprite, x as f64, 4f64);
        //cr.paint();

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

    fn compute_cell_size(&self, win_width: i32, win_height: i32){
        //---------------------------------------------------------
        //--
        let mut p = self.priv_.borrow_mut();
        //-- Calculer la taille du pixel affiché
        let o1 = (win_width-4) / (p.nb_columns+2);
        let o2 = (win_height-4) / p.nb_rows;
        if o1<o2 {
            p.cell_size = o1;
        }else{
            p.cell_size = o2;
        }

    }

    fn mouse_to_color(&self, mx: f64, my: f64)->u32 {
        //---------------------------------------------------------
        let p = self.priv_.borrow();
        let x: i32 = ((((mx-(2*p.cell_size) as f64) as i32)-4) / p.cell_size) as i32;
        let y: i32 = (((my as i32)-4) / p.cell_size) as i32;
        let id = (x + y * p.nb_columns) as usize;
        println!("Color id={}", id);
        p.colors[id] 
    }

    fn mouse_to_color_index(&self, mx: f64, my: f64)->usize {
        //---------------------------------------------------------
        let p = self.priv_.borrow();
        let x: i32 = ((((mx-(2*p.cell_size) as f64) as i32)-4) / p.cell_size) as i32;
        let y: i32 = (((my as i32)-4) / p.cell_size) as i32;
        let id = (x + y * p.nb_columns) as usize;
        println!("Color id={}", id);
        id
    }

    fn in_colors_zone(&self,mx: i32,my: i32)->bool {
        let p = self.priv_.borrow();
        ((mx>(2*p.cell_size))&&(my<(2*p.cell_size)))
        
    }

    fn save_colors(&self,file_name: &str){
        //--
        let path = Path::new(file_name);
        let display = path.display();

        let _ = match File::create(&path){
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => {
                
                let mut file = LineWriter::new(file);

                let p = self.priv_.borrow();

                let value = format!("{}\n",p.foreground_color.to_string());
                file.write_all(&value.as_bytes());

                let value = format!("{}\n",p.background_color.to_string());
                file.write_all(&value.as_bytes());

                for c in p.colors.iter(){
                    let value = format!("{}\n",c.to_string());
                    // println!("{} >> {}",file_name,value);
                    file.write_all(&value.as_bytes());
                    //file.write_all(b"\n");
                }

            },
        };
    }

    fn load_colors(&self,file_name: &str){
        //--
        let path = Path::new(file_name);
        let display = path.display();

        let _ = match File::open(&path){
            Err(why) => panic!("Couldn't open {}: {}", display, why),
            Ok(file) => {

                let mut p = self.priv_.borrow_mut();

                let f = io::BufReader::new(file);
                let mut i:usize = 0;
                let max_index = (p.nb_columns*p.nb_rows+2) as usize;
                for line in f.lines(){
                    if let Ok(ip) = line {
                        if (i==0){
                            p.foreground_color = ip.trim().parse::<u32>().unwrap();
                        }else if i==1 {
                            p.background_color = ip.trim().parse::<u32>().unwrap();
                        }else{
                            let c = ip.trim().parse::<u32>().unwrap();
                            p.colors[i-2] = c;    
                        }
                        //println!("<<<{}",ip);
                        i += 1;
                        if i>=max_index {
                            break;
                        }
                    }
                }
            },
        };

        // if let Ok(lines) = Palette::read_lines(file_name) {
        //     // Consumes the iterator, returns an (Optional) String
        //     for line in lines {
        //         if let Ok(ip) = line {
        //             println!("{}", ip);
        //         }
        //     }
        // }
    
    }    

}

impl Drop for PalettePrivate {
    fn drop(&mut self) {
       self.save_colors("palette.cfg");
    }
}

impl DrawingAreaImpl for PalettePrivate {}

impl WidgetImpl for PalettePrivate {

    fn draw(&self, _widget: &gtk::Widget, cr: &cairo::Context) -> Inhibit {
        //--
        let w = _widget.get_allocation().width - 1;
        let h = _widget.get_allocation().height - 1;

        //-- Compute Cells size
        self.compute_cell_size(w, h);

        // Examples are in 1.0 x 1.0 coordinate space
        //cr.scale(120.0, 120.0);

        //--
        self.draw_colors( &cr);

        //--
        Inhibit(false)
    }

    fn button_press_event(&self, _widget: &gtk::Widget, event: &gdk::EventButton) -> Inhibit {

        //let state: gdk::ModifierType = event.get_state();
        let widget1 = self.get_instance();

        //let b: bool = state.contains(CONTROL_MASK);

	    if (event.get_event_type()==gdk::EventType::DoubleButtonPress){ //-- Double Click
            if event.get_button()==1 {
                if let Some((x, y)) = event.get_coords() {
                    if self.in_colors_zone(x as i32, y as i32) {
                        let idcol: u32 = self.mouse_to_color_index( x, y) as u32;
                        widget1.emit("choose-color", &[&idcol]);            
                        //println!("Double Click x={} y={} idcol={}", x, y, idcol);
                    }
                }
            }

        }else{
            if event.get_button()==1 {
                if let Some((x, y)) = event.get_coords() {
                    let mut fore_color: u32=0;
                    let mut back_color: u32=0;
                    if self.in_colors_zone(x as i32, y as i32) {
                        let col = self.mouse_to_color( x, y);
                        let mut p = self.priv_.borrow_mut();
                        p.foreground_color = col;
                        widget1.queue_draw();
                        fore_color = p.foreground_color;
                        back_color = p.background_color;
                    
                    }else{
                        //-- Swap foreground an background colors
                        let mut p = self.priv_.borrow_mut();
                        let col = p.foreground_color;
                        p.foreground_color = p.background_color;
                        p.background_color = col;
                        fore_color = p.foreground_color;
                        back_color = p.background_color;
                    }
                    widget1.queue_draw();
                    widget1.emit("color-changed", &[&fore_color,&back_color]);

                    // if self.in_edit_area(px,py){
                    //     let p = self.priv_.borrow_mut();
                    //     p.sprite.put_pixel(px as u32, py as u32, 0, 0, 255, 255);
                    //     println!("Press x={} y={}", px, py);
                    //     widget1.queue_draw();
                    // }
                }            

            }else if event.get_button()==3 {
                if let Some((x, y)) = event.get_coords() {
                    let mut fore_color: u32=0;
                    let mut back_color: u32=0;
                    let mut f_color_changed: bool = false;
                    if self.in_colors_zone(x as i32, y as i32) {
                        let col = self.mouse_to_color( x, y);
                        let mut p = self.priv_.borrow_mut();
                        p.background_color = col;
                        widget1.queue_draw();
                        fore_color = p.foreground_color;
                        back_color = p.background_color;
                        f_color_changed = true;
                    }
                    if (f_color_changed){
                        widget1.emit("color-changed", &[&fore_color,&back_color]);
                    }
                    // if self.in_edit_area(px,py){
                    //     let p = self.priv_.borrow_mut();
                    //     p.sprite.put_pixel(px as u32, py as u32, 0, 0, 0, 0);
                    //     println!("Press x={} y={}", px, py);
                    //     widget1.queue_draw();
                    // }
                }            

            }
        }
        // if state==gdk::ModifierType::CONTROL_MASK {

        //     let mut p = self.priv_.borrow_mut();

        //     let mut nb_cols:i32 = 0;
        //     let mut nb_rows:i32 = 0;
        //     nb_cols = p.nb_pixs_columns;
        //     nb_rows = p.nb_pixs_rows;

        //     if event.get_button()==1 {
        //         --
        //         let w = self.get_instance();
        //         if let Some((x, y)) = event.get_coords() {
        //             println!("Press x={} y={}", x, y);
        //             let mx: u32 = x as u32;
        //             let my: u32 = y as u32;
        //             w.emit("left-clicked", &[&mx,&my]);
        //         }

        //     }

        //     let priv_ = self.priv_.borrow();

        //     println!("{} {}", nb_rows, nb_cols);
        //     p.nb_pixs_rows = nb_rows + 1;

       
        // }

        //--
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
        let state: gdk::ModifierType = event.get_state();
        let widget1 = self.get_instance();
        //let f:bool = state.contains(gdk::ModifierType::BUTTON1_MASK);
        //if state.contains(gdk::ModifierType::BUTTON1_MASK) && state.contains(gdk::ModifierType::CONTROL_MASK) {
        if state.contains(gdk::ModifierType::BUTTON1_MASK) {
            //--
            // if let Some((x, y)) = event.get_coords() {
            //     let (px,py) = self.mouse_to_pixel( x, y);
            //     if self.in_edit_area(px,py){
            //         let p = self.priv_.borrow_mut();
            //         p.sprite.put_pixel(px as u32, py as u32, 0, 0, 255, 255);
            //         widget1.queue_draw();
            //         //println!("Button Motion x={} y={}", x, y);
            //     }
            // }
        }else if state.contains(gdk::ModifierType::BUTTON3_MASK) {
            //--
            // if let Some((x, y)) = event.get_coords() {
            //     let (px,py) = self.mouse_to_pixel( x, y);
            //     if self.in_edit_area(px,py){
            //         let p = self.priv_.borrow_mut();
            //         p.sprite.put_pixel(px as u32, py as u32, 0, 0, 0, 0);
            //         widget1.queue_draw();
            //         //println!("Button Motion x={} y={}", x, y);
            //     }
            // }
        }else{
            if let Some((x, y)) = event.get_coords() {
                println!("Pointer Motion x={} y={}", x, y);
            }
        }

        //--
        Inhibit(false)
       
    }


}

glib_wrapper! {
    pub struct Palette(
        Object<subclass::simple::InstanceStruct<PalettePrivate>,
        subclass::simple::ClassStruct<PalettePrivate>,
        PaletteClass>)
        @extends gtk::DrawingArea, gtk::Widget;

    match fn {
        get_type => || PalettePrivate::get_type().to_glib(),
    }

}


impl Palette {

    pub fn new() -> Self {
        glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create Palette Widget")
            .downcast()
            .expect("Created MyWidget is of wrong type")
    }

    pub fn set_foreground_color(&self, col: u32, f_emit_signal: bool){
        let widget_priv = PalettePrivate::from_instance(self);
        let fore_color: u32;
        let back_color: u32;
        {
            let mut p = widget_priv.priv_.borrow_mut();
            p.foreground_color = col;
            self.queue_draw();
            fore_color = p.foreground_color;
            back_color = p.background_color;
        }
        if (f_emit_signal){
            self.emit("color-changed", &[&fore_color,&back_color]);
        }
    }

    pub fn set_background_color(&self, col: u32, f_emit_signal: bool){
        let widget_priv = PalettePrivate::from_instance(self);
        let fore_color: u32;
        let back_color: u32;
        {
            let mut p = widget_priv.priv_.borrow_mut();
            p.background_color = col;
            self.queue_draw();
            fore_color = p.foreground_color;
            back_color = p.background_color;
        }
        if (f_emit_signal){
            self.emit("color-changed", &[&fore_color,&back_color]);
        }
    }

    pub fn get_foreground_color(&self)->u32{
        let widget_priv = PalettePrivate::from_instance(self);
        let p = widget_priv.priv_.borrow();
        p.foreground_color

    }

    pub fn get_background_color(&self)->u32{
        let widget_priv = PalettePrivate::from_instance(self);
        let p = widget_priv.priv_.borrow();
        p.background_color
        
    }

    pub fn set_cell_color(&self, id: usize, col: u32){
        let widget_priv = PalettePrivate::from_instance(self);
        let mut p = widget_priv.priv_.borrow_mut();
        p.colors[id] = col;
        self.queue_draw();

    }

    pub fn get_cell_color(&self, id: usize)->u32{
        let widget_priv = PalettePrivate::from_instance(self);
        let p = widget_priv.priv_.borrow();
        p.colors[id]

    }

    pub fn save(&self,file_name: &str){
        //--
        let widget_priv = PalettePrivate::from_instance(self);
        widget_priv.save_colors(file_name);

    }

    // fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
    // where P: AsRef<Path>, {
    //     let file = File::open(filename)?;
    //     Ok(io::BufReader::new(file).lines())
    // }

    pub fn load(&self,file_name: &str){
        //--
        let widget_priv = PalettePrivate::from_instance(self);
        widget_priv.load_colors(file_name);
    
    }

}

pub trait PaletteExt {
    /// Connect to signal note-pressed
    fn connect_color_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;
    fn connect_choose_color<F: Fn(&Self, u32) + 'static>(&self, f: F) -> glib::SignalHandlerId;

}

impl PaletteExt for Palette {

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

