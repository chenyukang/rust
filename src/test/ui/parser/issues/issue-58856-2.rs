struct Empty;

trait Howness {}

impl Howness for () {
    fn how_are_you(&self -> Empty {
    //~^ ERROR expected parameter name
    //~^^ ERROR expected one of
    //~^^^ ERROR method `how_are_you` is not a member of trait `Howness`
    //~| ERROR associated function in `impl` without body
        Empty
    }
}
//~^ ERROR mismatched closing delimiter
//~| ERROR expected one of

fn main() {}
