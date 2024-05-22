use anchor_lang::prelude::*;

#[error_code]
pub enum MetapalError {
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
}