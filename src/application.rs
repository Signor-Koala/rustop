use gtk::{gio, glib, prelude::*, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    // By implementing Default we don't have to provide a `new` fn in our
    // ObjectSubclass impl.
    pub struct RustopApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for RustopApplication {
        const NAME: &'static str = "RustopApplication";
        type Type = super::RustopApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for RustopApplication {}
    impl ApplicationImpl for RustopApplication {
        fn activate(&self) {
            self.parent_activate();
            // We create our window at activation stage
            let window = gtk::ApplicationWindow::new(&*self.obj());
            window.set_default_size(600, 350);
            window.set_title(Some("Rustop"));

            let label = gtk::Label::new(Some("Hello"));
            label.add_css_class("title-2");
            window.set_child(Some(&label));
            window.present();
        }
    }
    impl GtkApplicationImpl for RustopApplication {}
}

glib::wrapper! {
    pub struct RustopApplication(ObjectSubclass<imp::RustopApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for RustopApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", "org.gtk_rs.application-subclass")
            .build()
    }
}
