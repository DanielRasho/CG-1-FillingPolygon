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
    framebuffer.set_background_color_hex(0x000000);
    framebuffer.clear();

    // STAR
    let star = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    framebuffer.fill_polygon(&star, Color::new(245, 200, 66));
    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0xFFFFFF);
    framebuffer.draw_polygon(&star);
    

    let vertices = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];
    framebuffer.fill_polygon(&vertices, Color::new(0, 0, 255));
    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0xFFFFFF);
    framebuffer.draw_polygon(&vertices);

    match framebuffer.render_buffer("./output.bmp") {
        Ok(_) => println!("Framebuffer rendered to output.bmp"),
        Err(e) => println!("Failed to render framebuffer: {}", e),
    }
}