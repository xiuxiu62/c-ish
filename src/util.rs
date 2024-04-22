#[macro_export]
macro_rules! match_one {
    ($value: ident, $($guard: expr),* $(,)?) => {
        $(matches!($value, $guard) || )*false
    };
}

#[macro_export]
macro_rules! match_all {
    ($value: ident, $($guard: expr),* $(,)?) => {
        $(matches!($value, $guard) && )*true
    };
}
