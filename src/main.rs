mod framebuffer;
mod line;
mod bmp;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::bmp::WriteBmp;
use crate::polygon::Polygon;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let polygon_1 = [(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360),
        (250, 380), (220, 385), (205, 410), (193, 383)];
    let polygon_2 = [(321, 335), (288, 286), (339, 251), (374, 302)];
    let polygon_3 = [(377, 249), (411, 197), (436, 249)];
    let polygon_4 = [(413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)];
    let polygon_5 = [(682, 175), (708, 120), (735, 148), (739, 170)];

    framebuffer.set_current_color(0xE5EDB2);
    framebuffer.scanline_fill(&polygon_1);
    framebuffer.set_current_color(0xFFB703);
    framebuffer.scanline_fill(&polygon_2);
    framebuffer.set_current_color(0x8ECAE6);
    framebuffer.scanline_fill(&polygon_3);
    framebuffer.set_current_color(0x90BE6D);
    framebuffer.scanline_fill(&polygon_4);
    framebuffer.set_current_color(0x000000);
    framebuffer.scanline_fill(&polygon_5);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.draw_polygon(&polygon_1);
    framebuffer.draw_polygon(&polygon_2);
    framebuffer.draw_polygon(&polygon_3);
    framebuffer.draw_polygon(&polygon_4);
    framebuffer.draw_polygon(&polygon_5);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
} 