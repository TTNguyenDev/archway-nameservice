use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSend { need: u128 },

    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    #[error("Name too short (len {len} min_length {min_length})")]
    NameTooShort { len: u64, min_length: u64 },

    #[error("Name too long (len {len} max_length {max_length})")]
    NameTooLong { len: u64, max_length: u64 },

    #[error("Invalid character(char {c}")]
    InvalidCharacter { c: char },
}
