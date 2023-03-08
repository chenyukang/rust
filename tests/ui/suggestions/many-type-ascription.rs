fn main() {
    let _ = type_ascribe(0, i32); //~ ERROR: type ascription is experimental
    let _ = type_ascribe(0, i32); // (error only emitted once)
}
