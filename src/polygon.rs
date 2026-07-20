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

        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;

        for &(_, y) in points {
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        for y in min_y..=max_y {
            let scanline_y = y as i32;
            let mut intersections = Vec::new();

            for i in 0..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[(i + 1) % points.len()];

                let y1i = y1 as i32;
                let y2i = y2 as i32;

                if y1i == y2i {
                    continue;
                }

                let (y_low, y_high) = if y1i < y2i {
                    (y1i, y2i)
                } else {
                    (y2i, y1i)
                };

                if scanline_y < y_low || scanline_y > y_high {
                    continue;
                }

                let x_intersection = x1 as f64
                    + ((scanline_y - y1i) as f64 * (x2 as f64 - x1 as f64)
                        / (y2i - y1i) as f64);

                intersections.push(x_intersection);
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for pair in intersections.chunks(2) {
                if pair.len() < 2 {
                    continue;
                }

                let start = pair[0].round() as i32;
                let end = pair[1].round() as i32;

                for x in start.min(end)..=start.max(end) {
                    self.point(x as usize, y);
                }
            }
        }
    }
}
