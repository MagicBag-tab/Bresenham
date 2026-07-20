use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[(usize, usize)]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 2 {
            return;
        }

        let n = points.len();
        for i in 0..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % n];
            self.draw_line(x1, y1, x2, y2);
        }
    }
}
