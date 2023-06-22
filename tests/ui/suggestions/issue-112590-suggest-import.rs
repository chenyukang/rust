pub struct S;

impl fmt::Debug for S { //~ ERROR failed to resolve: use of undeclared crate or module `fmt`
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { //~ ERROR failed to resolve: use of undeclared crate or module `fmt`
        //~^ ERROR failed to resolve: use of undeclared crate or module `fmt`
        Ok(())
    }
}

impl fmt::Invalid::Debug for S {} //~ ERROR failed to resolve: use of undeclared crate or module `fmt`

// will not suggest for not matching `Invalid`
impl fmt::Invalid for S {} //~ ERROR failed to resolve: use of undeclared crate or module `fmt`

type A = cell::RefCell; //~ ERROR failed to resolve: use of undeclared crate or module `cell`
type B = cell::RefError; //~ ERROR failed to resolve: use of undeclared crate or module `cell`

fn main() { }
