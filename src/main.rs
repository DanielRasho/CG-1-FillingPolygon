pub mod color;
pub mod framebuffer;
pub mod line;
pub mod poligon;

use color::Color;
use framebuffer::Framebuffer;
use framebuffer::Renderable;
use line::Line;
use poligon::Polygon;

use nalgebra_glm::Vec3;

fn main() {
     let mut framebuffer = Framebuffer::new_default(800, 500);
     
    // Clear the framebuffer with a white background
    framebuffer.set_background_color_hex(0xFFFFFF);
    framebuffer.clear();

    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0x000000);

    let vertices = vec![
        Vec3::new(10.0, 10.0, 0.0),
        Vec3::new(10.0, 100.0, 0.0),
        Vec3::new(20.0, 100.0, 0.0),
        Vec3::new(50.0, 50.0, 0.0),
        Vec3::new(80.0, 100.0, 0.0),
        Vec3::new(100.0, 100.0, 0.0),
        Vec3::new(100.0, 10.0, 0.0),
    ];

    framebuffer.fill_polygon(&vertices, Color::new(255, 0, 0));
    framebuffer.draw_polygon(&vertices);

    match framebuffer.render_buffer("./output.bmp") {
        Ok(_) => println!("Framebuffer rendered to output.bmp"),
        Err(e) => println!("Failed to render framebuffer: {}", e),
    }

}