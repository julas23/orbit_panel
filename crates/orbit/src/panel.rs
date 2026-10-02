//! The panel: app menu at the start, status widgets at the end.

use gtk::{glib, prelude::*};
use orbit_core::config::{MenuConfig, PanelConfig};

use crate::widgets;

pub fn build(app: &gtk::Application, config: &PanelConfig) -> gtk::Window {
    let orientation = if config.edge.is_vertical() {
        gtk::Orientation::Vertical
    } else {
        gtk::Orientation::Horizontal
    };

    let start = gtk::Box::new(orientation, 8);
    if let Some(menu) = &config.menu {
        start.append(&menu_button(menu));
    }

    let end = gtk::Box::new(orientation, 4);
    end.append(&widgets::clock::new());

    let root = gtk::CenterBox::builder()
        .orientation(orientation)
        .css_classes(["panel"])
        .start_widget(&start)
        .end_widget(&end)
        .build();

    gtk::ApplicationWindow::builder()
        .application(app)
        .title("Orbit Panel")
        .css_classes(["orbit", "orbit-panel"])
        .child(&root)
        .build()
        .upcast()
}

fn menu_button(menu: &MenuConfig) -> gtk::Button {
    let button = gtk::Button::builder().css_classes(["menu"]).build();
    match &menu.icon {
        Some(icon) => button.set_child(Some(&widgets::icon(icon, 26))),
        None => button.set_label("Apps"),
    }
    button.set_tooltip_text(menu.tooltip.as_deref());

    let command = menu.command.clone();
    button.connect_clicked(move |_| {
        if let Err(err) = glib::spawn_command_line_async(&command) {
            log::error!("running {command:?}: {err}");
        }
    });
    button
}
