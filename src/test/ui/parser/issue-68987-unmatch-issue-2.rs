use std::io::Error;
fn obstest() -> Result<(), Error> {
    let obs_connect = || -> Result<(), Error) {
        //~^ ERROR mismatched closing delimiter
        //~| ERROR expected one of
        if true { Ok(()) } else { Err(Error::last_os_error()) }
    };

    Ok(())
}

fn main() {}
