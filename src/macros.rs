#[macro_export]
macro_rules! doc_setter {
    ($tt:tt) => {
        $tt
        #[builder(setter(doc = stringify!($tt)))]
    };
}