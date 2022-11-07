struct Obj { //~ NOTE unmatched opening delimiter begins here
    member: usize
)
//~^ ERROR mismatched closing delimiter
//~| NOTE mismatched closing delimiter

fn main() {}
