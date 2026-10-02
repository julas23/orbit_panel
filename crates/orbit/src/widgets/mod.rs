pub mod clock;

use orbit_core::config::expand_home;

/// An image from either a file path or an icon-theme name.
pub fn icon(spec: &str, size: i32) -> gtk::Image {
    let image = if spec.contains('/') {
        gtk::Image::from_file(expand_home(spec.as_ref()))
    } else {
        gtk::Image::from_icon_name(spec)
    };
    image.set_pixel_size(size);
    image
}
