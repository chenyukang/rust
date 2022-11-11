// error-pattern: mismatched closing delimiter
mod unclosed_delim_mod;

fn main() {
    let _: usize = unclosed_delim_mod::new();
}
