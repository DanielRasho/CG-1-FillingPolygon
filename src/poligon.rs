use nalgebra_glm::Vec3;

use crate::{framebuffer::Framebuffer, line::Line, color::Color};

pub trait Polygon {
    fn draw_polygon(&mut self, vertices: &[Vec3]);
    fn fill_polygon(&mut self, vertices: &[Vec3], color: Color);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, vertices: &[Vec3]) {
        if vertices.len() < 3 {
            return; // Not enough vertices to form a polygon
        }

        for i in 0..vertices.len() {
            let v0 = vertices[i];
            let v1 = vertices[(i + 1) % vertices.len()]; // wrap around to the first vertex

            self.line(v0, v1);
        }
    }
    fn fill_polygon(&mut self, vertices: &[Vec3], color: Color) {
        let n = vertices.len();
        if n < 3 {
            return; // Not a valid polygon
        }

        // Detecting in which row the poligon start, and ends.
        let mut min_y = vertices[0].y as isize;
        let mut max_y = vertices[0].y as isize;
        for vertex in vertices.iter() {
            min_y = min_y.min(vertex.y as isize);
            max_y = max_y.max(vertex.y as isize);
        }

        
        // Creating an array of edges. Where the start of the 
        let mut edges: Vec<(isize, isize, isize, f32)> = Vec::new(); // (x1, y1, x2, slope)
        for i in 0..n {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % n];
            if v1.y != v2.y {
                let (x1, y1, x2, y2) = if v1.y < v2.y {
                    (v1.x as isize, v1.y as isize, v2.x as isize, v2.y as isize)
                } else {
                    (v2.x as isize, v2.y as isize, v1.x as isize, v1.y as isize)
                };
                let slope = (x2 as f32 - x1 as f32) / (y2 as f32 - y1 as f32);
                edges.push((x1, y1, y2, slope));
            }
        }
        
        for y in min_y..=max_y {
            // Determine the intersections of the current row with the edges calculated.
            let mut intersections: Vec<isize> = Vec::new();
            for &(x1, y1, y2, slope) in edges.iter() {
                if y >= y1 && y < y2 {
                    intersections.push(x1 + ((y - y1) as f32 * slope) as isize);
                }
            }
            // Sortin the edges to make sure they are from left to right.
            intersections.sort();

            // Iterating over the intersections by pairs, and filling with color 
            // Betweeen the the to pairs.
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x_start = intersections[i];
                    let x_end = intersections[i + 1];
                    for x in x_start..=x_end {
                        self.set_current_color(color);
                        self.draw_point(x as usize, y as usize);
                    }
                }
            }
        }
    }
}