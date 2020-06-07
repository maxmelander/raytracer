#[cfg(test)]

mod canvas_tests {
    use crate::canvas::*;
    use crate::color::Color;

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

        canvas.write_pixel(2, 3, red).unwrap();

        assert_eq!(canvas.get_color(2, 3), Ok(red));
    }

    #[test]
    fn ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("5 3"));
        assert_eq!(lines.next(), Some("255"));
    }

    #[test]
    fn ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1).unwrap();
        canvas.write_pixel(2, 1, c2).unwrap();
        canvas.write_pixel(4, 2, c3).unwrap();

        let ppm = canvas.to_ppm();
        assert_eq!(ppm, String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n"));
    }

    #[test]
    fn ppm_split_line() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        canvas.fill(color);

        let ppm = canvas.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("10 2"));
        assert_eq!(lines.next(), Some("255"));
        assert_eq!(lines.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 "));
        assert_eq!(lines.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153 "));
        assert_eq!(lines.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 "));
        assert_eq!(lines.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153 "));
    }
}
