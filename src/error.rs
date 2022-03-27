use thiserror::Error;

pub type Result<T> = std::result::Result<T, DnrError>;

#[derive(Error, Debug)]
pub enum DnrError {
    #[error("io/filesystem error")]
    IOError(#[from] std::io::Error),
    #[error("invalid user input: `{0}`")]
    InputError(String),
    #[error("failed to serialize")]
    SerializeError(#[from] serde_yaml::Error),
    #[error("failed to lock config file for write")]
    WriteError,
    #[error("unknown error occured: `{0}`")]
    Unknown(&'static str),
}
