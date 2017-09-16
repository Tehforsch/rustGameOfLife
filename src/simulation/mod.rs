extern crate rand;

pub mod cell_state;

use self::cell_state::CellState;

const WIDTH : usize = 40;
const HEIGHT: usize = 40;

pub struct Simulation {
    pub cells: [[CellState; WIDTH]; HEIGHT]
}

impl Simulation {
    pub fn timestep(&mut self) {
        let mut new_cells = [[CellState { is_alive: false }; WIDTH]; HEIGHT];
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let num_neighbors = self.count_neighbors(i, j);
                new_cells[i][j].is_alive = get_new_state(cell.is_alive, num_neighbors);
            }
        }
        self.cells = new_cells;
    }

    fn count_neighbors(&self, i: usize, j: usize) -> i64 {
        let mut num_alive_neighbors = 0;
        for &(neighbor_i, neighbor_j) in get_neighbors(i, j).iter() {
            if self.cells[neighbor_i][neighbor_j].is_alive {
                num_alive_neighbors += 1;
            }
        }
        num_alive_neighbors
    }

    pub fn from_random() -> Simulation {
        let mut sim = Simulation {
            cells: [[CellState { is_alive: false }; WIDTH]; HEIGHT]
        };
        for (_, row) in sim.cells.iter_mut().enumerate() {
            for (_, cell) in row.iter_mut().enumerate() {
                cell.is_alive = rand::random::<bool>();
            }
        }
        sim
    }
}

fn get_new_state(is_alive: bool, num_alive_neighbors: i64) -> bool{
    if is_alive {
        num_alive_neighbors == 2 || num_alive_neighbors == 3
    }
    else {
        num_alive_neighbors == 3
    }
}

fn get_neighbors(i: usize, j: usize) -> [(usize, usize); 8] {
    // Adding WIDTH-1 is the same as subtracting one (and subtracting one can lead to
    // difficulties with negative overflows in the usize type
    [
        ((i+1) % WIDTH, (j+1) % HEIGHT),
        ((i+1) % WIDTH, j),
        ((i+1) % WIDTH, (j+(HEIGHT-1)) % HEIGHT),
        (i, (j+1) % HEIGHT),
        (i, (j+(HEIGHT-1)) % HEIGHT),
        ((i+(WIDTH-1)) % WIDTH, (j+1) % HEIGHT),
        ((i+(WIDTH-1)) % WIDTH, j),
        ((i+(WIDTH-1)) % WIDTH, (j+(HEIGHT-1)) % HEIGHT),
    ]
}

