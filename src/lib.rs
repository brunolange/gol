extern crate web_sys;
extern crate js_sys;

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Timer<'a> {
    name: &'a str,
}

impl <'a> Timer <'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl <'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let mut cells = FixedBitSet::with_capacity((width*height) as usize);
        for i in 0..cells.len() {
            cells.set(i, js_sys::Math::random() >= 0.5);
            // cells.set(i, i%2 == 0 || i%7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                let cell = self.cells[i];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (true, n) if n < 2 || n > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                next.set(i, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        let state = self.cells[idx];
        self.cells.set(idx, !state);
    }
}

impl Universe {

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

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
