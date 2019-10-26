use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::ptr;

extern crate gtk;
extern crate gdk;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    WindowPosition,
    Label,
    Button,
    Entry,
    ListBox,
    ListBoxRow,
    Popover,
    PopoverExt,
    PositionType,
    ScrolledWindow,
    Adjustment,
    Settings,
};
use gdk::{
    Screen,
    Display,
    Window as GdkWindow,
    WindowExt as GdkWindowExt,
    WindowAttr as GdkWindowAttr,
    WindowType as GdkWindowType,
    WindowWindowClass,
    // Cursor,
};

const STYLES: &'static str = "
* {
    /* -gtk-dpi: 300; */
    -gtk-icon-theme: \"DMZ-Mac-Black\";
}

#box {
    margin: 15px;
}

/*
list {
    border-radius: 8px;
}

row {
    padding: 20px 20px;
}

button {
    padding: 20px 25px;
    outline: none;
}

button, entry {
    border-radius: 8px;
    border-width: 3px;
    outline-width: 2px;
}
*/

button.li-menu-button {
    background: transparent;
    border: none;
    font-size: 15px;
    letter-spacing: 7px;
}

popover {
    padding: 10px;
}

entry {
    padding: 10px 15px;
}
";

use std::ffi;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use gdk_sys::GdkDisplay;

#[repr(C)]
struct XDisplay {}

extern "C" {
    // fn gdk_x11_get_default_xdisplay() -> *mut XDisplay;
    // fn gdk_x11_lookup_xdisplay(xdisplay: *mut XDisplay) -> *mut GdkDisplay;
    // fn gdk_x11_display_set_cursor_theme(display: *mut GdkDisplay, theme: *const c_char, size: c_int);
}

fn main() -> Result<(), ()> {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        return Ok(());
    }

    // Set new cursor theme
    // unsafe {
    //     // Get displays
    //     let xdisplay = gdk_x11_get_default_xdisplay();
    //     let gdk_display = gdk_x11_lookup_xdisplay(xdisplay);

    //     // Theme name
    //     let theme_name = CString::new("Bibata_Amber").unwrap();

    //     // Set cursor theme
    //     gdk_x11_display_set_cursor_theme(gdk_display, theme_name.as_c_str().as_ptr(), 96);
    // };

    // Set new default cursor
    // let def_display = Display::get_default().unwrap();
    // let cursor = Cursor::new_from_name(&def_display, "default");
    
    let screen_width = Screen::width();
    let screen_height = Screen::height();
    let window = Window::new(WindowType::Toplevel);
    window.set_title("TODO Attempt");

    let btn = Button::new_with_label("Add Item");
    let text = Entry::new();
    text.set_placeholder_text(Some("I will..."));
    text.set_hexpand(true);
    
    let list = ListBox::new();
    list.set_vexpand(true);
    let list_scroll = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    list_scroll.add(&list);

    let gbox = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    let wrap = gtk::Box::new(gtk::Orientation::Vertical, 20);
    gbox.add(&text);
    gbox.add(&btn);
    gbox.set_hexpand(true);
    gtk::WidgetExt::set_name(&wrap, "box");

    let quit_btn = Button::new_with_label("Close");
    let quit_btn_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    quit_btn_box.pack_end(&quit_btn, false, false, 0);
    wrap.add(&gbox);
    wrap.add(&list_scroll);
    wrap.add(&quit_btn_box);

    window.set_position(WindowPosition::CenterOnParent);
    window.set_default_size(screen_width / 3, screen_height / 3);
    window.add(&wrap);
    window.show_all();

    let win = window.clone();
    // let set_cursor = RefCell::new(false);
    // window.connect_draw(move |_, _| {
    //     {
    //         let setc = set_cursor.borrow();
    //         if *setc == false {
    //             let gdkw = win.get_window().unwrap();
    //             gdkw.set_cursor(cursor.as_ref());
    //             win.present();
    //         }
    //     }

    //     set_cursor.replace(true);
    //     Inhibit(false)
    // });
    
    let weak_list = list.downgrade();

    btn.connect_clicked(move |_| {
        println!("Add item");

        let label_text_gstr = text.get_text().unwrap();
        let label_text = label_text_gstr.as_str();
        let list = weak_list.upgrade().unwrap();

        if label_text.len() != 0 {
            let row = ListBoxRow::new();
            let row_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
            let opt_menu = Button::new_with_label("•••");
            let label = Label::new(Some(label_text));
            row_box.pack_start(&label, false, false, 0);
            row_box.pack_end(&opt_menu, false, false, 0);
            row.add(&row_box);
            list.add(&row);
            list.show_all();

            // Clear text
            text.set_text("");
            text.grab_focus();

            // Hide button's bg & stuff
            let om_style_ctx = opt_menu.get_style_context();
            om_style_ctx.add_class("li-menu-button");

            let popo = Popover::new(Some(&opt_menu));
            let popo_del = Button::new_with_label("Delete");
            let popo_dup = Button::new_with_label("Copy");
            let popo_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

            popo_box.add(&popo_del);
            popo_box.add(&popo_dup);
            popo.add(&popo_box);
            popo.set_position(PositionType::Bottom);
            popo.show_all();
            popo.hide();

            let popo_copy = popo.clone();

            opt_menu.connect_clicked(move |_| {
                popo.popup();
            });

            popo_del.connect_clicked(move |_| {
                list.remove(&row);
                popo_copy.popdown();
            });
        }
    });

    quit_btn.connect_clicked(|_| {
        gtk::main_quit();
    });

    println!("Screen width: {}px", screen_width);
    println!("Screen height: {}px", screen_height);

    // Add CSS
    let css_prov = gtk::CssProvider::new();
    css_prov
        .load_from_data(STYLES.as_bytes())
        .expect("Failed to load CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &css_prov,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    gtk::main();
    Ok(())
}

