use std::cell::RefCell;
use gtk::gdk::RGBA;
use std::rc::Rc;
use core::f64::consts::PI;
use crate::glib::clone;
use gtk::DrawingArea;
use gtk::Orientation;
use gtk::Picture;
use gtk::cairo::Context;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn draw_arcs(drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32) {
    let allocation = drawing_area.allocation();
    let width = allocation.width() as f64;
    let height = allocation.height() as f64;
    let lesser = width.min(height);

    // Coordinates for the center of the window
    let xc = width / 2.0;
    let yc = height / 2.0;

    context.set_line_width(lesser * 0.02); // outline thickness changes with window size

    // First draw a simple unclosed arc
    let _ = context.save();
    context.arc(width / 3.0, height / 4.0, lesser / 4.0, -(PI / 5.0), PI);
    context.close_path(); // line back to start point
    context.set_source_rgb(0.0, 0.8, 0.0);
    let _ = context.fill_preserve();
    let _ = context.restore(); // back to opaque black
    let _ = context.stroke();

    // Now draw a circle
    let _ = context.save();
    context.arc(xc, yc, lesser / 4.0, 0.0, 2.0 * PI); // full circle
    context.set_source_rgba(0.0, 0.0, 0.8, 0.6); // partially translucent
    let _ = context.fill_preserve();
    let _ = context.restore(); // back to opaque black
    let _ = context.stroke();

    // And finally an ellipse
    let ex = xc;
    let ey = 3.0 * height / 4.0;
    let ew = 3.0 * width / 4.0;
    let eh = height / 3.0;

    let _ = context.save();
    context.translate(ex, ey); // make (ex, ey) == (0, 0)
    context.scale(ew / 2.0, eh / 2.0); // for width: ew / 2.0 == 1.0 for height: eh / 2.0 == 1.0
    context.arc(0.0, 0.0, 1.0, 0.0, 2.0 * PI); // 'circle' centered at (0, 0) with 'radius' of 1.0
    context.set_source_rgba(0.8, 0.0, 0.0, 0.7);
    let _ = context.fill_preserve();
    let _ = context.restore(); // back to opaque black
    let _ = context.stroke();
}

fn draw_arc(drawing_area: &DrawingArea, context: &Context, width: i32, height: i32) {

    context.arc(
        width as f64 / 2.0,
        height as f64 / 2.0,
        (width.min(height)) as f64 / 2.0,
        0.0,
        2.0 * PI,
    );

    context.set_source_rgba(0.3, 0.5, 0.1, 0.7);
    let _ = context.fill();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Episode Four: Draw and Display a Picture that changes when key is pressed")
        .build();

    let drawing_area = DrawingArea::new();
    let picture = Picture::new();
    let mut vertical_box = gtk::Box::new(Orientation::Vertical, 0);
    vertical_box.append(&drawing_area);
    vertical_box.append(&picture);
    window.set_child(Some(&vertical_box));

    let event_controller_key = gtk::EventControllerKey::new();
    let mut counter: i32 = 0;
    set_display_content(counter, &mut vertical_box);
    counter += 1;
    let counter_rc = Rc::new(RefCell::new(counter));

    let vertical_box_rc = Rc::new(RefCell::new(vertical_box));

    event_controller_key.connect_key_pressed(clone!(@strong counter_rc, @strong window, @strong vertical_box_rc => move |_, key, _, _| {
        if let Some(key_name) = key.name() {
            match key_name.as_str() {
                "q" => window.close(),
                _ => {
                    if let Ok(mut counter) = counter_rc.try_borrow_mut() {
                        if let Ok(mut vertical_box) = vertical_box_rc.try_borrow_mut() {
                            set_display_content(*counter, &mut *vertical_box);
                            *counter += 1;
                        }
                    }
                },
            }
        };
        gtk::Inhibit(false)

    }));
    window.add_controller(event_controller_key);


    // Present window
    window.present();
}

fn set_display_content(counter: i32, vertical_box: &mut gtk::Box) {
    println!("set display content: counter:{}", counter);
    while let Some(child) = vertical_box.first_child() {
        vertical_box.remove(&child)
    }
    if counter % 2 == 0 {
        let drawing_area = DrawingArea::new();
        drawing_area.set_content_width(100);
        drawing_area.set_content_height(100);
        drawing_area.set_draw_func(draw_arcs);

        let picture = Picture::new();
        picture.set_hexpand(true);
        picture.set_vexpand(true);
        picture.set_filename(Some("testdata/paul-klee-revolution-of-the-viaduct.jpeg"));

        vertical_box.append(&drawing_area);
        vertical_box.append(&picture);
    } else {
        let drawing_area = DrawingArea::new();
        drawing_area.set_content_width(100);
        drawing_area.set_content_height(100);
        drawing_area.set_draw_func(draw_arc);

        let picture = Picture::new();
        picture.set_hexpand(true);
        picture.set_vexpand(true);
        picture.set_filename(Some("testdata/maurits-cornelis-escher-gecko.jpeg"));

        vertical_box.append(&drawing_area);
        vertical_box.append(&picture);
    }
}


