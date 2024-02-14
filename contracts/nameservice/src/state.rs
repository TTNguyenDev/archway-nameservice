use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
    pub price: Coin,
}

// Define state variables
pub const PURCHASE_PRICE: Item<Coin> = Item::new("purchase_price");
pub const NAME_RECORDS: Map<&[u8], NameRecord> = Map::new("name_records");
