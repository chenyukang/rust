fn main() {
    let _t = vec::new(); //~ ERROR failed to resolve
    type B = vec::Vec::<u8>; //~ ERROR failed to resolve
    let _t = std::sync_error::atomic::AtomicBool; //~ ERROR failed to resolve
}
