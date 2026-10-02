# Orbit Panel

A desktop panel and dock for Linux, written in **Rust** with **GTK4** and fully themeable through **CSS**.

Orbit Panel is a ground-up rewrite of an existing panel/dock setup built on [EWW](https://github.com/elkowar/eww). The goal is to keep the flexibility of CSS-based styling while gaining the performance, type safety, and maintainability of a native Rust application.

> **Status:** early development. Nothing is usable yet — APIs, configuration format, and features are all subject to change.

## Goals

- **Native and lightweight** — a single Rust binary, no scripting runtime or polling shell scripts.
- **CSS-first theming** — the entire look and feel is defined in a user stylesheet, with live reload.
- **Panel + dock** — a top/bottom panel and an application dock, both anchored to screen edges.
- **Wayland-native** — built on the layer-shell protocol via `gtk4-layer-shell`.
- **Easy migration from EWW** — familiar concepts and styling for users coming from an EWW-based setup.

## Planned features

### Panel
- Workspaces indicator
- Focused window title
- Clock and calendar
- System tray
- Audio, network, battery, and other status modules

### Dock
- Pinned and running applications
- Running/focused indicators
- Click to launch or focus

## Tech stack

| Component      | Choice                                                                 |
| -------------- | ---------------------------------------------------------------------- |
| Language       | [Rust](https://www.rust-lang.org/)                                     |
| UI toolkit     | [GTK4](https://gtk.org/) via [gtk4-rs](https://gtk-rs.org/)            |
| Layer shell    | [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell)           |
| Styling        | GTK CSS                                                                |

## Building

> Build instructions will be added once the initial scaffold lands.

Expected requirements:

- Rust (stable) and Cargo
- GTK4 development libraries
- `gtk4-layer-shell` development libraries
- A Wayland compositor that supports the `wlr-layer-shell` protocol

## Contributing

Contributions, ideas, and feedback are welcome. As the project is in its early stages, please open an issue to discuss larger changes before submitting a pull request.

## License

Orbit Panel is licensed under the [GNU General Public License v3.0](LICENSE).
