use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use adw::subclass::prelude::*;

use crate::config::VERSION;
use crate::{
    GameOfLifeWindow,
    widgets::GameOfLifePreferencesWindow,
    services::GameOfLifeSettings
};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GameOfLifeApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for GameOfLifeApplication {
        const NAME: &'static str = "GameOfLifeApplication";
        type Type = super::GameOfLifeApplication;
        type ParentType = adw::Application;

        fn new() -> Self {
            Self::default()
        }
    }

    impl ObjectImpl for GameOfLifeApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.preferences", &["<ctrl>comma"]);
            obj.set_accels_for_action("win.play", &["p"]);
            obj.set_accels_for_action("win.snapshot", &["<ctrl>s"]);
            obj.set_accels_for_action("win.open-snapshot", &["<ctrl>o"]);
            obj.set_accels_for_action("win.toggle-design-mode", &["e"]);
            obj.set_accels_for_action("win.new", &["<ctrl>n"]);
            obj.set_accels_for_action("win.new-empty", &["<ctrl>e"]);
            obj.set_accels_for_action("win.random-seed", &["<ctrl>r"]);
        }
    }

    impl ApplicationImpl for GameOfLifeApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = GameOfLifeWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for GameOfLifeApplication {}
    impl AdwApplicationImpl for GameOfLifeApplication {}
}

glib::wrapper! {
    pub struct GameOfLifeApplication(ObjectSubclass<imp::GameOfLifeApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GameOfLifeApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create GameOfLifeApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);

        let preferences_action = gio::SimpleAction::new("preferences", None);
        preferences_action.connect_activate(clone!(@weak self as app => move |_, _| {
            let preferences_window = GameOfLifePreferencesWindow::new(app.active_window().as_ref());
            preferences_window.show();
        }));
        self.add_action(&preferences_action);

        let disable_design_hint_action = gio::SimpleAction::new("disable-design-hint", None);
        disable_design_hint_action.connect_activate(clone!(@weak self as app => move |_, _| {
            GameOfLifeSettings::default().set_show_design_hint(false);
        }));
        self.add_action(&disable_design_hint_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("Game of Life")
            .version(VERSION)
            .authors(vec!["Andrea Coronese".into()])
            .build();

        dialog.present();
    }
}

