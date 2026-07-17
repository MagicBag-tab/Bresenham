use crate::framebuffer::Framebuffer;

pub trait Line {
    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
}

impl Line for Framebuffer {
    fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = -(y1 as i32 - y0 as i32).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        
        let mut err = dx + dy;

        let mut x = x0 as i32;
        let mut y = y0 as i32;

        loop {
            self.point(x as usize, y as usize);

            if x == x1 as i32 && y == y1 as i32 {
                break;
            }

            let e2 = 2 * err;
            
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}