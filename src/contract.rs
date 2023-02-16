#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// Burada versiyonlarimizi yaratiyor ki ileride migrate edersek kontrati bu bilgileri kullanabilelim
const CONTRACT_NAME: &str = "crates.io:template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // Once state i yaratiyoruz
    let state = State {
    };

    // Kontrat versiyonumuzu kaydediyoruz.
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Yarattigimiz state i blockchain e kalici olarak kaydediyoruz.
    STATE.save(deps.storage, &state)?;

    // Fonksiyonumuz donus yapiyor burada, return degerleri olarak methodun ne oldugunu ve instantiate eden kisinin kim oldugunu donduruyoruz.
    // Bu degerler ozellikle front end kisminda onemli olacak.
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Burada match icinde Execute mesajlari ve onlarin dogru fnksiyonlara yonlendirilmeleri olacak.
    match msg {
    }
}

pub mod execute {
    use super::*;
    // Burada match icinde Execute mesajlarinin karsilik gelecegi fonksiyonlar olacak.
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // Burada match icinde Query mesajlari ve onlarin dogru fnksiyonlara yonlendirilmeleri olacak.
    Ok(Binary::default())
}

pub mod query {
    use super::*;
    // Burada match icinde Query mesajlarinin karsilik gelecegi fonksiyonlar olacak.
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn init_test() {
        // Asagidaki kod sahte deps yaratiyor bizim icin
        let mut deps = mock_dependencies();

        // Init mesajimiz
        let msg = InstantiateMsg {  };

        // Sahte deps lerimizle kullanilabilmesi icin info yaratiyoruz.
        // Normalde bu info, kullanan adresten geliyor ama su anda sahte bir ortamda oldugumuzdan ve kendi kendimize test ettigimizden, info yu kendimiz yaratiyoruz.
        let info = mock_info("creator", &coins(1000, "earth"));

        // Init fonksiyonunu cagirip, return degerini res te tutuyoruz.
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        // Donen degerin uzunlugu var mi diye bakiyoruz calisip calismadigini anlamak icin.
        assert_eq!(0, res.messages.len());
    }
}
