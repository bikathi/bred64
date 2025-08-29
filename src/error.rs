use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncoderError {
    #[error("failed to calculate encoded length: {0}")]
    LengthCalculationError(#[from] std::num::TryFromIntError),
    #[error("character not found in base64 table")]
    InvalidCharacter,
    #[error("encoder error: {0}")]
    General(String),
}
