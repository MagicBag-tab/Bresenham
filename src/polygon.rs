use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[(usize, usize)]);
    fn scanline_fill(&mut self, points: &[(usize, usize)]);
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

    fn scanline_fill(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;

        for &(x, y) in points {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        let seed_x = ((min_x + max_x) / 2).min(self.width.saturating_sub(1));
        let seed_y = ((min_y + max_y) / 2).min(self.height.saturating_sub(1));

        self.flood_fill(seed_x, seed_y);
    }
}
