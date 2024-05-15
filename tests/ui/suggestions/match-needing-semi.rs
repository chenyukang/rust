// check-only

fn main() {
    match 3 {
        4 => 1,
        3 => {
            foo() //~ ERROR mismatched types
        }
        _ => 2
    }
    let _ = ();
}

fn foo() -> i32 {
    42
}
