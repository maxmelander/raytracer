#[cfg(test)]
use super::canvas::*;
use super::color::*;

#[test]
fn create_canvas() {
    let canvas = Canvas::new(10, 20);
    let data = canvas.data();

    for r in data {
        for c in r {
            assert_eq!(*c, Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn write_pixel() {
    let mut canvas = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);

    canvas.write_pixel(2, 3, red);

    assert_eq!(canvas.get_color(2, 3), Ok(red));
}
