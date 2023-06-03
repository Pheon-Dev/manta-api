use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    // -- Key
    KeyFailHmac,

    // -- Pwd
    PwdNotMatching,
    SchemeUnknown(String),
    SchemeNotFoundInContent,
    // -- TODO: Token
}

// region: --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter,
    ) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "self:?")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
