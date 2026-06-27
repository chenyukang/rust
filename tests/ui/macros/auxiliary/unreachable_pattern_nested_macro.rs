#[macro_export]
macro_rules! create_pats {
    () => {
        macro_rules! pat_a {
            () => {
                _
            };
        }
        macro_rules! pat_b {
            () => {
                _
            };
        }
    };
}
