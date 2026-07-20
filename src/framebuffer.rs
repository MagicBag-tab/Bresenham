pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn flood_fill(&mut self, start_x: usize, start_y: usize) {
        let mut stack = vec![(start_x, start_y)];
        let fill_color = self.current_color;
        let boundary_color = self.background_color;

        while let Some((x, y)) = stack.pop() {
            if x >= self.width || y >= self.height {
                continue;
            }

            let idx = y * self.width + x;
            if self.buffer[idx] != boundary_color {
                continue;
            }

            self.buffer[idx] = fill_color;

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x + 1 < self.width {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y + 1 < self.height {
                stack.push((x, y + 1));
            }
        }
    }
}