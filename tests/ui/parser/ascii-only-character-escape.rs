fn main() {
    let x = "\x80"; //~ ERROR out of range hex escape
    let y = "\xff"; //~ ERROR out of range hex escape
    let z = "\xe2"; //~ ERROR out of range hex escape
    let a = b"\x00e2";  // ok because byte literal

    // from issue #148917
    dbg!('\xFF'); //~ ERROR out of range hex escape
}
