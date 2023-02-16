use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // Burada sizin belirleyeceginiz hata cesitleri olacak.
    // Ornegin, asagidaki custom bir unauthorized hatasi yaratiyor.
    #[error("Unauthorized")]
    Unauthorized {},
}
