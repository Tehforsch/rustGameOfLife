mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context};

use self::draw::square;
use simulation::Simulation;
use point::Point;

const CELL_DRAW_SIZE: f64 = 5.0;

pub fn render(context: Context, gl: &mut GlGraphics, sim: &mut Simulation) {
    piston_window::clear([0.0, 0.0, 0.0, 1.0], gl);
    for (i, row) in sim.cells.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let color = {
                if cell.is_alive {
                    [0.3, 0.0, 0.0, 1.0]
                }
                else {
                    [0.0, 0.0, 0.3, 1.0]
                }
            };
            square(
                Point {
                    x: (i as f64) * CELL_DRAW_SIZE, 
                    y: (j as f64) * CELL_DRAW_SIZE
                }, 
                CELL_DRAW_SIZE, 
                color, 
                context, 
                gl);
        }
    }
}
