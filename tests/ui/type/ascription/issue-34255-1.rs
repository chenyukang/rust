struct Reactor {
    input_cells: Vec<usize>,
}

impl Reactor {
    pub fn new() -> Self {
        input_cells: Vec::new()
        //~^ ERROR failed to resolve: use of undeclared crate or module
    }
}

// This case isn't currently being handled gracefully, including for completeness.
fn main() {}
