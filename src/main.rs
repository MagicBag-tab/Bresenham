mod framebuffer;
mod line;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::bmp::WriteBmp;
use crate::line::Line;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.draw_line(0, 0, 799, 599);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
} 