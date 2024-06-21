mod application;

use application::RustopApplication;
use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let app = RustopApplication::default();
    app.run()
}
