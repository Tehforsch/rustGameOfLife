use piston_window::ellipse::Ellipse;
use piston_window::rectangle;
use piston_window::{Context, Transformed};
use piston_window::types::Color;
use opengl_graphics::GlGraphics;

use ::point::Point;

#[allow(dead_code)]

pub fn circle(pos: Point, radius: f64, color: Color, context: Context, gl: &mut GlGraphics) {
    Ellipse {
            color: color,
            border: None,
            resolution: 16,
    }.draw(
        [0.0, 0.0, 2.0*radius, 2.0*radius],
        &Default::default(),
        context.trans(pos.x-radius, pos.y-radius).transform,
        gl);
}

pub fn square(pos: Point, size: f64, color: Color, context: Context, gl: &mut GlGraphics) {
    let center = context.transform.trans(pos.x, pos.y);
    rectangle(color, [0.0, 0.0, size, size], center, gl);
}
