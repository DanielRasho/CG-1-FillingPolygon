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
    

    let square = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];
    framebuffer.fill_polygon(&square, Color::new(0, 0, 255));
    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0xFFFFFF);
    framebuffer.draw_polygon(&square);

    let triangle = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0),
    ];
    framebuffer.fill_polygon(&triangle, Color::new(255, 0, 0));
    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0xFFFFFF);
    framebuffer.draw_polygon(&triangle);

    let teapot = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0),
    ];
    framebuffer.fill_polygon(&teapot, Color::new(0, 255, 0));
    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0xFFFFFF);
    framebuffer.draw_polygon(&teapot);


    let hole = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0),
    ];
    framebuffer.fill_polygon(&hole, Color::new(0, 0, 0));
    // Set the current drawing color to black
    framebuffer.set_current_color_hex(0xFFFFFF);
    framebuffer.draw_polygon(&hole);

    match framebuffer.render_buffer("./output.bmp") {
        Ok(_) => println!("Framebuffer rendered to output.bmp"),
        Err(e) => println!("Failed to render framebuffer: {}", e),
    }
}