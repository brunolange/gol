mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (self.width * row + column) as usize
    }

    // cell neighbors as iterator?
    // fn neighbors(&self, row: u32, col: u32) {
    // }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for dr in [self.height - 1, 0, 1].iter().cloned() {
            for dc in [self.width - 1, 0, 1].iter().cloned() {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = (row + dr) % self.height;
                let nc = (col + dc) % self.width;
                let i = self.get_index(nr, nc);
                count += self.cells[i] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead => '◻',
                    Cell::Alive => '◼',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "{}", "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i%2 == 0 || i%7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect()
        ;

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                let cell = self.cells[i];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, n) if n < 2 || n > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[i] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}
