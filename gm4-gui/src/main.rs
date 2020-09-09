mod app;
mod effects;
mod profiles;

fn main() {
    app::run();
}

pub fn arr_to_rgba(arr: [u8; 3]) -> gdk::RGBA {
    gdk::RGBA {
        red: arr[0] as f64 / 255.0,
        green: arr[1] as f64 / 255.0,
        blue: arr[2] as f64 / 255.0,
        alpha: 1.0,
    }
}

pub fn rgba_to_arr(rgba: gdk::RGBA) -> [u8; 3] {
    [
        (rgba.red * 255.0).round() as u8,
        (rgba.green * 255.0).round() as u8,
        (rgba.blue * 255.0).round() as u8,
    ]
}
