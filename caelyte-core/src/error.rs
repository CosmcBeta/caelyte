use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaelyteError {
    #[error("Temp Error")]
    TempError,
}
