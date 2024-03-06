//! 
//!
//! 
//! 
//! 

#[macro_use]
extern crate glib;

//extern crate gio;
//extern crate gtk;
//extern crate gdk_pixbuf;

use std::mem::transmute;
use gtk::prelude::*;
use gio::prelude::*;
//use glib::clone;

use std::env::args;
use std::str;
use std::path::PathBuf;

extern crate once_cell;

// use gio::subclass::prelude::*;
// use gio::ApplicationFlags;
// use gio::Resource;
// use glib::Type;
// use glib::subclass;
// use glib::subclass::prelude::*;
// use glib::translate::*;
// use gtk::subclass::prelude::*;
// use gtk::subclass::widget::WidgetImplExt;
// use once_cell::unsync::OnceCell;
// use std::cell::{Cell, RefCell};
// use gdk_pixbuf::*;


use gdk::{RGBA, Cursor, Display};

//use gtk::gdk;

use gtk::{ApplicationWindow, Builder,AccelFlags, AccelGroup, AboutDialog, MessageDialog, MenuItem,
    Orientation, Box,  DestDefaults, TargetFlags,
    ResponseType, FileChooserAction, FileChooserDialog, ColorChooserDialog, Dialog,
    Window, IconTheme, IconThemeExt, IconLookupFlags, ToolButton};


mod rgb_utils; // Nécessaire ici seulement
use crate::rgb_utils::{get_rgba, rgba, get_rgba_r, get_rgba_g, get_rgba_b, get_rgba_a};

mod rpoint;

mod rrect;
use rrect::{RRect};

mod select_rect;
use select_rect::{SelectRect};

mod pencil_mode;
mod rectangle_mode;
mod ellipse_mode;
mod select_mode;
mod fill_mode;

mod edit_area;
use edit_area::{EditArea, EditAreaExt};

mod palette;
use palette::{Palette, PaletteExt};

mod sprites_bar;
use sprites_bar::{SpritesBar, SpritesBarExt};

use std::rc::Rc;


// lazy_static! {
//      static sprites_bar2: SpritesBar = SpritesBar::new();
// }

// use std::collections::HashMap;
// lazy_static! {
//     static ref HASHMAP: HashMap<u32, &'static str> = {
//         let mut m = HashMap::new();
//         m.insert(0, "hello");
//         m.insert(1, ",");
//         m.insert(2, " ");
//         m.insert(3, "world");
//         m
//     };
//     static ref COUNT: usize = HASHMAP.len();
// }

fn about_clicked(item: &MenuItem, dialog: &AboutDialog) {
    if let Some(window) = item
        .get_toplevel()
        .and_then(|w| w.downcast::<Window>().ok())
    {
        dialog.set_transient_for(Some(&window));
    }

    // We only want to hide the dialog when it's closed and not completely destroy it
    // as otherwise we can't show it again a second time.
    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        gtk::Inhibit(true)
    });

    dialog.connect_response(|dialog, _| {
        dialog.hide();
    });

    println!("Authors: {:?}", dialog.get_authors());
    println!("Artists: {:?}", dialog.get_artists());
    println!("Documenters: {:?}", dialog.get_documenters());

    dialog.show_all();
}

fn save_as(sprites_bar1: &SpritesBar){

    if let Some(window) = sprites_bar1.get_toplevel().and_then(|w| w.downcast::<Window>().ok()) {

        let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
                                            FileChooserAction::Save);
        dialog.add_buttons(&[
            ("Save", ResponseType::Ok),
            ("Cancel", ResponseType::Cancel)
        ]);

        dialog.set_select_multiple(false);
        dialog.set_modal(true);

        let response: ResponseType = dialog.run();

        if response == ResponseType::Ok {
            let files = dialog.get_filenames();
            let file_name = &files[0].to_str();
            println!("Files: {:?}", file_name);
            sprites_bar1.save_as_sprite(file_name.unwrap());
        }

        dialog.close();

    }

}

static mut sprite_width : i32 = 32;
static mut sprite_height : i32 = 32;

fn build_ui(application: &gtk::Application) {
    
    //glib-compile-resources.exe  --sourcedir res myapp.gresource.xml
    let resources_bytes = include_bytes!("myapp.gresource");
    let resource_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resource_data).unwrap();
    gio::resources_register(&res);
    let icontheme = IconTheme::get_default().unwrap();
    icontheme.add_resource_path("/res");
 
    //let glade_src = include_str!("gtk_sprite_ed.glade");
    //let builder = Builder::from_string(glade_src);
    let builder = Builder::from_resource("/res/gtk_sprite_ed.glade");

    let window: ApplicationWindow = builder.get_object("Window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    window.set_title("GTK+ Sprite Editor 2");
    window.set_border_width(4);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 800);
   
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);


    // let dialog: MessageDialog = builder
    //     .get_object("messagedialog1")
    //     .expect("    messagedialog1");
    // dialog.connect_delete_event(|dialog, _| {
    //     dialog.hide();
    //     gtk::Inhibit(true)
    // });

    // window.connect("key_press_event", false, |values| {
    //     // "values" is a 2-long slice of glib::value::Value, which wrap G-types
    //     // You can unwrap them if you know what type they are going to be ahead of time
    //     // values[0] is the window and values[1] is the event
    //     let raw_event = &values[1].get::<gdk::Event>().unwrap().unwrap();
    //     // You have to cast to the correct event type to access some of the fields
    //     match raw_event.downcast_ref::<gdk::EventKey>() {
    //         Some(event) => {
    //             println!("PRESS>>>>>>>>>>>>>>>>>>>>>");
    //             println!("key value: {:?}", event.get_keyval());
    //             println!("modifiers: {:?}", event.get_state());
    //         },
    //         None => {},
    //     }

    //     // You need to return Some() of a glib Value wrapping a bool
    //     let result = glib::value::Value::from_type(glib::types::Type::Bool);
    //     // I can't figure out how to actually set the value of result
    //     // Luckally returning false is good enough for now.
    //     Some(result)
    // })
    // .unwrap();

    // window.connect("key_release_event", false, |values| {
    //     // "values" is a 2-long slice of glib::value::Value, which wrap G-types
    //     // You can unwrap them if you know what type they are going to be ahead of time
    //     // values[0] is the window and values[1] is the event
    //     let raw_event = &values[1].get::<gdk::Event>().unwrap().unwrap();
    //     // You have to cast to the correct event type to access some of the fields
    //     match raw_event.downcast_ref::<gdk::EventKey>() {
    //         Some(event) => {
    //             println!("RELEASE<<<<<<<<<<<<<<<<<<<<<");
    //             println!("key value: {:?}", event.get_keyval());
    //             println!("modifiers: {:?}", event.get_state());
    //         },
    //         None => {},
    //     }

    //     // You need to return Some() of a glib Value wrapping a bool
    //     let result = glib::value::Value::from_type(glib::types::Type::Bool);
    //     // I can't figure out how to actually set the value of result
    //     // Luckally returning false is good enough for now.
    //     Some(result)
    // })
    // .unwrap();


    let sprites_bar1 = SpritesBar::new();
    let spr1 = sprites_bar1.get_sprite();
    //spr1.fill(0xFF0000FF);

    let edit_area1 = EditArea::new();
    edit_area1.set_sprite(spr1);
    let palette1 = Palette::new();
    palette1.load("palette.cfg");

    //-- Synchronize selected colors with palette widget
    edit_area1.set_colors(palette1.get_foreground_color(), palette1.get_background_color());
    edit_area1.set_pencil_mode();

    // edit_area1.connect_key_press_event(|_, _| { // Don't work => need Widget with Window ?
    //     println!("key pressed");
    //     Inhibit(false)
    // });

    //--
    let tool_select: ToolButton = builder.get_object("SelectMode").expect("Couldn't get SelectMode");
    let tool_pencil: ToolButton = builder.get_object("PencilMode").expect("Couldn't get PencilMode");
    let tool_rectangle: ToolButton = builder.get_object("RectangleMode").expect("Couldn't get RectangleMode");
    let tool_ellipse: ToolButton = builder.get_object("EllipseMode").expect("Couldn't get EllipseMode");
    let tool_fill: ToolButton = builder.get_object("FloodFillMode").expect("Couldn't get FloodFillMode");
    let tool_current: ToolButton = builder.get_object("CurrentMode").expect("Couldn't get CurrentMode");

    tool_select.connect_clicked(clone!( @weak edit_area1, @weak tool_current => move |_| {
        edit_area1.set_select_mode();
        tool_current.set_icon_name(Some("SelectBoxIcon"));
    }));
    
    tool_pencil.connect_clicked(clone!( @weak edit_area1, @weak tool_current => move |_| {
        edit_area1.set_pencil_mode();
        tool_current.set_icon_name(Some("PencilIcon"));
    }));

    tool_rectangle.connect_clicked(clone!( @weak edit_area1, @weak tool_current => move |_| {
        edit_area1.set_rectangle_mode();
        tool_current.set_icon_name(Some("RectangleIcon"));
    }));

    tool_ellipse.connect_clicked(clone!( @weak edit_area1, @weak tool_current => move |_| {
        edit_area1.set_ellipse_mode();
        tool_current.set_icon_name(Some("EllipseIcon"));
    }));

    tool_fill.connect_clicked(clone!( @weak edit_area1, @weak tool_current => move |_| {
        edit_area1.set_fill_mode();
        tool_current.set_icon_name(Some("FloodFillIcon"));
    }));

    //--
    let hbox1: gtk::Box = builder.get_object("HBox1").expect("Couldn't get HBox1");
    hbox1.pack_start(&edit_area1, true, true, 0);
    hbox1.pack_end(&sprites_bar1, false, true, 0);    

    //--
    let vbox2: gtk::Box = builder.get_object("VBox2").expect("Couldn't get VBox2");
    vbox2.pack_end(&palette1, false, true, 0);

    let item_quit : MenuItem = builder.get_object("menu_item_quit").expect("Couldn't get menu_item_quit");
    item_quit.connect_activate(clone!(@weak window => move |_| {
        window.close();
    }));

    let item_new : MenuItem = builder.get_object("menu_item_new").expect("Couldn't get menu_item_new");
    item_new.connect_activate(clone!(@weak sprites_bar1 => move |_| {
        if let Some(window) = sprites_bar1.get_toplevel().and_then(|w| w.downcast::<Window>().ok()) {

            let dialog = Dialog::with_buttons(Some("Enter Size"),
                        Some(&window),
                        gtk::DialogFlags::MODAL,
                        &[("OK", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel)]);
            
            dialog.set_default_size(200,128);

            let hbox1 = gtk::Box::new(Orientation::Horizontal, 2);
            let width_label = gtk::Label::new(Some("Width"));
            let width_entry = gtk::Entry::new();
            width_entry.set_alignment(1.);
            width_entry.set_width_chars(6);
            unsafe{
                width_entry.set_text(&format!("{}",sprite_width));
            }
            hbox1.pack_start(&width_label, false, false, 1);
            hbox1.pack_end(&width_entry, false, false, 1);

            let hbox2 = gtk::Box::new(Orientation::Horizontal, 2);
            let height_label = gtk::Label::new(Some("Height"));
            let height_entry = gtk::Entry::new();
            height_entry.set_alignment(1.);
            height_entry.set_width_chars(6);
            unsafe{
                height_entry.set_text(&format!("{}",sprite_height));
            }
            hbox2.pack_start(&height_label, false, false, 1);
            hbox2.pack_end(&height_entry, false, false, 1);

            let vbox1 = gtk::Box::new(Orientation::Vertical, 2);
            vbox1.pack_start(&hbox1, false, false, 1);
            vbox1.pack_start(&hbox2, false, false, 1);


            dialog.get_content_area().pack_start(&vbox1, false, false, 2);

            dialog.connect_response(glib::clone!(@weak sprites_bar1, @weak width_entry, @weak height_entry => move |dialog, response| {
                //entry.set_text(&format!("Clicked {}", response));
                if response == ResponseType::Ok {
                    let w = match width_entry.get_text().parse::<u32>(){
                                Ok(w) => w,
                                _ => 0
                            };
                    let h = match height_entry.get_text().parse::<u32>(){
                        Ok(h) => h,
                        _ => 0
                    };
                    if (w!=0) && (h!=0) {
                        unsafe{
                            sprite_width = w as i32;
                            sprite_height = h as i32;
                        }
                        println!("Width = {} Height = {}", w, h);
                        sprites_bar1.new_sprite(w as i32, h as i32);
                    }
                }
                dialog.close();
            }));
            dialog.show_all();
        }

    }));


    let item_open : MenuItem = builder.get_object("menu_item_open").expect("Couldn't get menu_item_open");
    item_open.connect_activate(clone!(@weak sprites_bar1 => move |_| {
        
        if let Some(window) = sprites_bar1.get_toplevel().and_then(|w| w.downcast::<Window>().ok()) {

            let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
                                            FileChooserAction::Open);
            dialog.add_buttons(&[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel)
            ]);

            dialog.set_select_multiple(false);

            dialog.connect_response(clone!( @weak sprites_bar1  => move |dialog, response| {
                if response == ResponseType::Ok {
                    let files = dialog.get_filenames();
                    let file_name = &files[0].to_str();
                    println!("Files: {:?}", file_name);
                    sprites_bar1.load_sprite(file_name.unwrap());
                }
                dialog.close();
            }));

            dialog.show_all();
        }

    }));

    let item_save : MenuItem = builder.get_object("menu_item_save").expect("Couldn't get menu_item_save");
    item_save.connect_activate(clone!(@weak sprites_bar1 => move |_| {

        let file_name = sprites_bar1.get_file_name();
        if file_name==""{
            println!("Call Save As ");
            save_as(&sprites_bar1);

        }else{
            println!("Save {}",file_name);
            sprites_bar1.save_sprite();
        
        }

    }));


    let item_save_as : MenuItem = builder.get_object("menu_item_save_as").expect("Couldn't get menu_item_save_as");
    item_save_as.connect_activate(clone!(@weak sprites_bar1 => move |_| {
        println!("Menu Item Save As");
        save_as(&sprites_bar1);

        // if let Some(window) = edit_area1.get_toplevel().and_then(|w| w.downcast::<Window>().ok()) {
        //     let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
        //                                         FileChooserAction::Save);
        //     dialog.add_buttons(&[
        //         ("Save", ResponseType::Ok),
        //         ("Cancel", ResponseType::Cancel)
        //     ]);
        //     dialog.set_select_multiple(false);
        //     dialog.set_modal(true);

        //     dialog.connect_response(clone!( @weak edit_area1 => move |dialog, response| {
        //         if response == ResponseType::Ok {
        //             let files = dialog.get_filenames();
        //             println!("Files: {:?}", files);
        //             let file_name = &files[0].to_str();
        //             println!("Files: {:?}", file_name);
        //             edit_area1.save_as_sprite(file_name.unwrap());
        //         }
        //         dialog.close();
        //     }));
        //     dialog.show_all();
        
        // }

    }));

    let item_flip_horizontaly : MenuItem = builder.get_object("menu_item_flip_horizontaly").expect("Couldn't get menu_item_flip_horizontaly");
    item_flip_horizontaly.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Flip Horizontaly");
        edit_area1.flip_horizontaly();

    }));

    let item_flip_verticaly : MenuItem = builder.get_object("menu_item_flip_verticaly").expect("Couldn't get menu_item_flip_verticaly");
    item_flip_verticaly.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Flip Verticaly");
        edit_area1.flip_verticaly();
    }));

    let item_swing90left : MenuItem = builder.get_object("swing90left").expect("Couldn't get swing90left");
    item_swing90left.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Swing 90 Left");
        edit_area1.swing_left();
    }));

    let item_swing90right : MenuItem = builder.get_object("swing90right").expect("Couldn't get swing90right");
    item_swing90right.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Swing 90 Right");
        edit_area1.swing_right();
    }));


    let item_copy : MenuItem = builder.get_object("menu_item_copy").expect("Couldn't get menu_item_copy");
    item_copy.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Copy");
        edit_area1.edit_copy();
    }));
    let (key, modifier) = gtk::accelerator_parse("<Primary>C");
    item_copy.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let item_paste : MenuItem = builder.get_object("menu_item_paste").expect("Couldn't get menu_item_paste");
    item_paste.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Paste");
        edit_area1.edit_paste();
    }));
    let (key, modifier) = gtk::accelerator_parse("<Primary>V");
    item_paste.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let item_cut : MenuItem = builder.get_object("menu_item_cut").expect("Couldn't get menu_item_cut");
    item_cut.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Cut");
        edit_area1.edit_cut();
    }));
    let (key, modifier) = gtk::accelerator_parse("<Primary>X");
    item_cut.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let item_undo : MenuItem = builder.get_object("menu_item_undo").expect("Couldn't get menu_item_undo");
    item_undo.connect_activate(clone!(@weak edit_area1 => move |_| {
        println!("Menu Item Undo");
        edit_area1.edit_undo();
    }));
    let (key, modifier) = gtk::accelerator_parse("<Primary>Z");
    item_undo.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let dialog_about: AboutDialog = builder.get_object("AboutDialog1").expect("Couldn't get dialog");

    let item_about : MenuItem = builder.get_object("menu_item_about").expect("Couldn't get menu_item_about");
    item_about.connect_activate(move |x| about_clicked(x, &dialog_about));

    window.show_all();

    //let win = window.get_window().unwrap(); // Need to be call here : after window show

    // if let Some(display) = gdk::Display::get_default() {
    //     let pixbuf1 = Pixbuf::from_resource("/res/Swing90RightIcon.png").unwrap();
    //     let cursor1 = Cursor::from_pixbuf(&display, &pixbuf1, 0, 0);
    //     win.set_cursor(Some(&cursor1));
    // }    

    // let win = window.get_window().unwrap(); // Need to be call here : after window show
    // if let Some(display) = gdk::Display::get_default() {
    //     win.set_cursor(Some(&Cursor::new_for_display(
    //         &display,
    //         gdk::CursorType::Cross,
    //     )));
    // }

    sprites_bar1.connect_sprite_changed(
        clone!(@weak edit_area1 => move |s,i_sel_sprite|{
            let spr1 = s.get_sprite();
            edit_area1.set_sprite(spr1);
            //println!("Refresh Edit area ......");
        }),
    );

    // {
    //     let w = edit_area1.get_window().unwrap();
    //     if let Some(display) = gdk::Display::get_default() {
    //         w.set_cursor(Some(&Cursor::new_for_display(
    //             &display,
    //             gdk::CursorType::Boat,
    //         )));
    //     }        
    // }

    edit_area1.grab_focus();
    // edit_area1.connect_key_press_event(
    //     |s,e| {
    //         println!("Key Press Edit Area1...");
    //         // let w = s.get_window().unwrap();
    //         // if let Some(display) = gdk::Display::get_default() {
    //         //     w.set_cursor(Some(&Cursor::new_for_display(
    //         //         &display,
    //         //         gdk::CursorType::Boat,
    //         //     )));
    //         // }        
    //         //--
    //         Inhibit(false)
    //     },
    // );

    // edit_area1.connect_leave_notify_event(
    //     {
    //         let pal1 = palette1.clone();
    //         move |s, e| 
    //         {
    //             //--
    //             let c = pal1.get_foreground_color();
    //             let w = s.get_window().unwrap();
    //             if let Some(display) = gdk::Display::get_default() {
    //                 w.set_cursor(Some(&Cursor::new_for_display(
    //                     &display,
    //                     gdk::CursorType::CoffeeMug,
    //                 )));
    //             }                
    //             println!("Leave notify Edit Area1...");
    //             //--
    //             Inhibit(false)
    //         }
    //     }
    // );


    edit_area1.connect_edit_changed(
        clone!(@weak sprites_bar1 => move |_|{ 
            sprites_bar1.fresh_display();
            //println!("Refresh Sprites Bar ......");
        }),
    );

    edit_area1.connect_sprite_transform(
        clone!(@weak sprites_bar1 => move |e|{
            let spr1 = e.get_sprite();
            sprites_bar1.set_sprite(spr1);
            sprites_bar1.fresh_display();
            //println!("Refresh Sprites Bar ......");
        }),
    );
    
    edit_area1.connect_color_changed(
        clone!(@weak window => move |_, fore_col, back_col|{ 
            //edit_area1.set_colors(fore_col, back_col);
            //println!("color-changed {} {}", fore_col, back_col);
        }),
    );

    edit_area1.connect_pick_foreground_color(
        clone!(@weak palette1 => move |e,fore_col|{
            palette1.set_foreground_color(fore_col, false);
            //println!("Pick Foreground Color {}",fore_col);
        }),
    );

    edit_area1.connect_pick_background_color(
        clone!(@weak palette1 => move |e,fore_col|{
            palette1.set_background_color(fore_col, false);
            //println!("Pick Foreground Color {}",fore_col);
        }),
    );

    // Configure the text view to accept URI lists from other applications. This allows
    // dragging files & folders from a file browser program onto the textview.
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        TargetFlags::OTHER_APP,
        0,
    )];
    edit_area1.drag_dest_set(DestDefaults::ALL, &targets, gdk::DragAction::COPY|gdk::DragAction::LINK|gdk::DragAction::DEFAULT);
    //println!("Connect Drag Data Received");

    // Process any `drag-data-received` events received by the textview. These events include
    // the URL list we're looking for.
    edit_area1.connect_drag_data_received(
        {
            let sprites_bar2 = sprites_bar1.clone();
            move |w, _, _, _, d, _, _| {
                if d.get_length()>0 {

                    for uri in d.get_uris() {
                        
                        let str_uri = uri.to_string();

                        if let Ok((pathbuf,_)) = glib::filename_from_uri(&str_uri) {

                            if let Some(suffixe) = pathbuf.extension() {
                                if suffixe=="png" {
                                    if let Some(filepath) =  pathbuf.to_str() {
                                        sprites_bar2.load_sprite(filepath);
                                        //println!("Drag Data Received {:?}", filepath);
                                    }
                                }
                            }

                        }

                    }

                    // let data = d.get_data();
                    // let s = match str::from_utf8(&data) {
                    //     Ok(v) => v,
                    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    // };
                    // if s.contains("file:///") {
                    //     let mut l =s.len()-3;
                    //     let ss = &s[8..l];
                    //     l = ss.len();
                    //     if &ss[l-4..]==".png" {
                    //     sprites_bar2.load_sprite(ss);
                    //     println!("Drag Data Received {:?}",ss);
                    //     }
                    // }

                }
            }
        }
    );

    palette1.connect_color_changed(
        clone!(@weak edit_area1 => move |_, fore_col, back_col|{ 
            edit_area1.set_colors(fore_col, back_col);
            println!("color-changed {} {}", fore_col, back_col);
        }),
    );

    palette1.connect_choose_color(

        clone!(@weak palette1 => move |_, idcol|{ 

            if let Some(window) = palette1.get_toplevel().and_then(|w| w.downcast::<Window>().ok()) {

                let dialog = ColorChooserDialog::new(Some("Choose Color"), Some(&window));
                let sel_col = palette1.get_cell_color(idcol as usize);
                let gdk_rgba = RGBA{
                    red: (get_rgba_r(sel_col) as f64)/255.0,
                    green: (get_rgba_g(sel_col) as f64)/255.0,
                    blue: (get_rgba_b(sel_col) as f64)/255.0,
                    alpha: (get_rgba_a(sel_col) as f64)/255.0,
                };
                dialog.set_rgba(&gdk_rgba);
                dialog.connect_response(clone!( @weak palette1 => move |dialog, response| {
                    if response == ResponseType::Ok {
                        println!("Choose Color idcol={}",idcol);
                        let choose_color = dialog.get_rgba();
                        let r = (choose_color.red * 255.0) as u8;
                        let g = (choose_color.green * 255.0) as u8;
                        let b = (choose_color.blue * 255.0) as u8;
                        let a = (choose_color.alpha * 255.0) as u8;
                        palette1.set_cell_color(idcol as usize, rgba(r,g,b,a));
                        palette1.set_foreground_color(rgba(r,g,b,a), true);
                        palette1.save("palette.cfg");
                    }
                    dialog.close();
                }));
                dialog.show_all();

            }
            
            //println!("color-changed idcol={}", idcol);
        }),

    );

}

// fn execute_closure<F>(closure_argument: F)
// where
//     F: Fn() -> i32,
// {
//     let result = closure_argument();
//     println!("Result of closure: {}", result);
// }

fn main() {

    // let test = RefCell::new(String::new());
    // execute_closure(|| {
    //     let mut s = test.borrow_mut();
    //     s.push_str("Azzzzeee");
    //     10
    // });
    // let s = test.borrow();
    // println!("{:?}",s);

    // let mut tbl : [Rc<Pixbuf>;8];
    // tbl = [
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap()),
    //     Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap())
    // ];

    // let sprite1 = tbl[0].clone();
    // sprite1.fill(0x00000000);

    // tbl[0] = Rc::new(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32).unwrap());

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.gtk3_sprite_ed"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

}
