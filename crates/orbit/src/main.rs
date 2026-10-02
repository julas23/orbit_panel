mod dock;
mod panel;
mod widgets;

use std::{
    cell::RefCell,
    path::{Path, PathBuf},
};

use anyhow::Context;
use gtk::{gdk, glib, prelude::*};
use orbit_core::{Backend, Config, Placement, style::Style};

const APP_ID: &str = "io.github.julas23.Orbit";
const BUILTIN_CSS: &str = include_str!("../../../data/style.css");

fn main() -> glib::ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config_path = match parse_args() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("orbit: {err}");
            return glib::ExitCode::FAILURE;
        }
    };

    let app = gtk::Application::builder().application_id(APP_ID).build();
    // Holds the stylesheet watcher for the lifetime of the application.
    let style = RefCell::new(None);
    app.connect_activate(move |app| {
        // A second `orbit` invocation re-activates the running instance.
        if !app.windows().is_empty() {
            return;
        }
        match start(app, &config_path) {
            Ok(s) => *style.borrow_mut() = Some(s),
            Err(err) => {
                log::error!("{err:#}");
                app.quit();
            }
        }
    });
    // Arguments are already parsed; don't let GApplication reject them.
    app.run_with_args::<&str>(&[])
}

fn parse_args() -> anyhow::Result<PathBuf> {
    let mut args = std::env::args().skip(1);
    let mut config = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" | "--config" => {
                config = Some(PathBuf::from(args.next().context("--config needs a path")?));
            }
            "-h" | "--help" => {
                println!("Usage: orbit [-c|--config PATH]");
                std::process::exit(0);
            }
            other => anyhow::bail!("unknown argument: {other}"),
        }
    }
    Ok(config.unwrap_or_else(Config::default_path))
}

fn start(app: &gtk::Application, config_path: &Path) -> anyhow::Result<Style> {
    let config = Config::load(config_path)?;
    let display = gdk::Display::default().context("no display")?;
    let backend = backend_for(&display)?;
    log::info!("using the {} backend", backend.name());

    let style = Style::install(&display, BUILTIN_CSS, &config.style_path(config_path));

    // The dock is created first so the panel stacks above it where they meet,
    // as in a left panel + top dock layout.
    if config.dock.enabled {
        let dock = &config.dock;
        let window = dock::build(app, dock);
        let placement = Placement {
            edge: dock.edge,
            size: dock.size,
            monitor: dock.monitor,
            reserve: dock.reserve,
        };
        backend.attach(&window, placement)?;
        window.present();
    }
    if config.panel.enabled {
        let panel = &config.panel;
        let window = panel::build(app, panel);
        let placement = Placement {
            edge: panel.edge,
            size: panel.size,
            monitor: panel.monitor,
            reserve: panel.reserve,
        };
        backend.attach(&window, placement)?;
        window.present();
    }
    Ok(style)
}

fn backend_for(display: &gdk::Display) -> anyhow::Result<Box<dyn Backend>> {
    if display.is::<gdk4_x11::X11Display>() {
        return Ok(Box::new(orbit_backend_x11::X11Backend::new()?));
    }
    if display.is::<gdk4_wayland::WaylandDisplay>() {
        return Ok(Box::new(orbit_backend_wayland::WaylandBackend::new()?));
    }
    anyhow::bail!("unsupported display backend: {}", display.type_().name())
}
