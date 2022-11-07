// FIXME(31528) we emit a bunch of silly errors here due to continuing past the
// first one. This would be easy-ish to address by better recovery in tokenisation.

pub fn trace_option(option: Option<isize>) {
    option.map(|some| 42;
                          //~^ ERROR: closure bodies that contain statements must be surrounded by braces

}
//~^ ERROR: expected expression, found `)`
//~^^ ERROR: mismatched closing delimiter

fn main() {}
