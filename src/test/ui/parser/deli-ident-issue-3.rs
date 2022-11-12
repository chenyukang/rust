fn main() {
    let a = [}; //~ ERROR mismatched closing delimiter
    let b = (}; //~ ERROR mismatched closing delimiter
    let c = {]; //~ ERROR mismatched closing delimiter
} //~ ERROR unexpected closing delimiter
