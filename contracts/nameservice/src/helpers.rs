use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;

const MIN_LENGTH: u64 = 3;
const MAX_LENGTH: u64 = 64;

pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<Option<&Coin>, ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let mut sent_sufficient_funds = sent.iter().filter_map(|coin| {
                if coin.denom == required_coin.denom && coin.amount.u128() > required_amount {
                    Some(coin)
                } else {
                    None
                }
            });
            if let Some(coin) = sent_sufficient_funds.next() {
                return Ok(Some(coin));
            } else {
                return Err(ContractError::InsufficientFundsSend {});
            }
        }
    }
    Ok(None)
}

pub fn validate_name(name: &str) -> Result<(), ContractError> {
    let len = name.len() as u64;
    if len < MIN_LENGTH {
        Err(ContractError::NameTooShort {
            len,
            min_length: MIN_LENGTH,
        })
    } else if len > MAX_LENGTH {
        Err(ContractError::NameTooLong {
            len,
            max_length: MAX_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(b) => {
                let c = name[b..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}

pub fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}
