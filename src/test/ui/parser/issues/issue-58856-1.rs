impl A {
    //~^ ERROR cannot find type `A` in this scope
    fn b(self>
    //~^ ERROR expected parameter name
    //~^^ ERROR expected one of
    //~| ERROR associated function in `impl` without body
}
//~^ ERROR mismatched closing
//~| ERROR expected one of

fn main() {}
