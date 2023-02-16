use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    // Buradaki mesajiniz ile instantiate edilecek kontrat.
}

#[cw_serde]
pub enum ExecuteMsg {
    // Burada blockchain e yazmak icin kullanacaginiz mesajlar olacak.
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Burada blockchainden data almak ciin kullanacaginiz fonksiyonlar olacak.
}
