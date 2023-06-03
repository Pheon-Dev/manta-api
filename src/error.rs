use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // -- Modules
    // TODO: Crypt
    Model(model::Error),
}

// TODO: Crypt constructor implementation

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Error::Model(val)
    }
}

// region: --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
