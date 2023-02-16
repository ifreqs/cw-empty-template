use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    // Burada kontratinizin state i tutulacak
}

// Burada State i, blockchain e kaydediyoruz. 
// Bu sekilde buradaki datalar blockchain de kalici olarak kaliyor.
pub const STATE: Item<State> = Item::new("state");