pub trait ConstStr {
    const STR: &'static str;
}
macro_rules! impl_const_str {
    ($ty:ident, $str:expr) => {
        impl $crate::traits::ConstStr for $ty {
            const STR: &'static str = $str;
        }
    };
}
