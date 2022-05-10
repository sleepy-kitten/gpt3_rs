pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error)
}
macro_rules! convert_error {
    ($variant: expr, $error: ty) => {
        impl From<$error> for Error {
            fn from(error: $error) -> Error {
                $variant(error)
            }
        }
    };
}
convert_error!(Error::Reqwest, reqwest::Error);
convert_error!(Error::Serde, serde_json::Error);