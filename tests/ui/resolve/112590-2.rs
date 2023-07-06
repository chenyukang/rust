// run-rustfix
fn main() {
    //let _t: Vec<i32> = vec::new(); //~ ERROR failed to resolve
    //B = vec::Vec::<u8>; //~ ERROR failed to resolve
    let _t = std::sync_error::atomic::AtomicBool::new(true); //~ ERROR failed to resolve
}
