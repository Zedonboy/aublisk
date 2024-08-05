use candid::{Nat, Principal};
use ic_cdk::{api::call::{self, call_with_payment}, call, id};
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::{
        account::Account,
        transfer::{TransferArg, TransferError},
    },
};
use sha2::{Digest, Sha224};

use crate::types::Metadata;
const EXCHANGE_RATE_MAINNET_CANISTER: &str = "uf6dk-hyaaa-aaaaq-qaaaq-cai";
const STAKING_PRINCIPAL :&str = "";
const TOKEN_PRINCIPAL : &str = "";
pub async fn transfer_to_account(
    token_principal: Principal,
    amt_to_transfer: u64,
    subaccount: Option<[u8; 32]>,
    to_account: Account
) -> Result<(), String> {
    let transfer_args = TransferArg {
        memo: None,
        amount: Nat::from(amt_to_transfer),
        fee: None,
        from_subaccount: subaccount,
        to: to_account,
        created_at_time: None,
    };

    let (resp,): (Result<Nat, TransferError>,) = call_with_payment(
        token_principal,
        "icrc1_transfer",
        (transfer_args,),
        20_000_000,
    )
    .await
    .unwrap();
    let _ = resp.map_err(|err| err.to_string())?;
    Ok(())
}
pub async fn transfer_to_purse(
    token_principal: Principal,
    amt_to_transfer: u64,
    subaccount: Option<[u8; 32]>,
) -> Result<(), String> {
    let canister_purse = Account {
        owner: id(),
        subaccount: Some([0; 32]),
    };

   transfer_to_account(token_principal, amt_to_transfer, subaccount, canister_purse).await
}

pub async fn transfer_to_staking_purse(amt : u64, caller : Principal) -> Result<(), String> {
    let staking_principal = Principal::from_text(STAKING_PRINCIPAL).unwrap();
    let token_principal = Principal::from_text(TOKEN_PRINCIPAL).unwrap();
    let to_account = Account { owner: id(), subaccount: Some(principal_to_subaccount(&staking_principal)) };
    //TODO(Substract Transaction fee)
   transfer_to_account(token_principal, amt, Some(principal_to_subaccount(&caller)), to_account).await
}
pub fn principal_to_subaccount(principal: &Principal) -> [u8; 32] {
    let mut subaccount = [0; 32];
    let principal_bytes = principal.as_slice();
    let hash = Sha224::digest(principal_bytes);
    subaccount[0..28].copy_from_slice(&hash);
    subaccount
}

pub async fn get_amount_of_token_per_dollar(
    token_symbol: String,
    decimal: usize,
) -> Result<u64, String> {
    let arg = GetExchangeRateRequest {
        base_asset: Asset {
            symbol: "USD".to_string(),
            class: AssetClass::FiatCurrency,
        },
        quote_asset: Asset {
            symbol: token_symbol,
            class: AssetClass::Cryptocurrency,
        },
        timestamp: None,
    };
    let (result,): (GetExchangeRateResult,) = call_with_payment(
        Principal::from_text(EXCHANGE_RATE_MAINNET_CANISTER).unwrap(),
        "get_exchange_rate",
        (arg,),
        50_000_000,
    )
    .await
    .unwrap();

    if result.is_err() {
        let err = result.unwrap_err();
        let mssg = match err {
            ic_xrc_types::ExchangeRateError::AnonymousPrincipalNotAllowed => "Anonymous user",
            ic_xrc_types::ExchangeRateError::Pending => "Pending Request, try again",
            ic_xrc_types::ExchangeRateError::CryptoBaseAssetNotFound => {
                "Asset not found in the market for price determination"
            }
            ic_xrc_types::ExchangeRateError::CryptoQuoteAssetNotFound => {
                "Asset not found in the market for price determination"
            }
            ic_xrc_types::ExchangeRateError::StablecoinRateNotFound => {
                "Dollar rate of the asset cannot be determined"
            }
            ic_xrc_types::ExchangeRateError::StablecoinRateTooFewRates => "Very Few Dollar rate",
            ic_xrc_types::ExchangeRateError::StablecoinRateZeroRate => "Dollar zero rate",
            ic_xrc_types::ExchangeRateError::ForexInvalidTimestamp => "Invalid Timestamp",
            ic_xrc_types::ExchangeRateError::ForexBaseAssetNotFound => "Base Asset not found",
            ic_xrc_types::ExchangeRateError::ForexQuoteAssetNotFound => "Quote asset not found",
            ic_xrc_types::ExchangeRateError::ForexAssetsNotFound => "Assets not found",
            ic_xrc_types::ExchangeRateError::RateLimited => "Rate Limited",
            ic_xrc_types::ExchangeRateError::NotEnoughCycles => {
                "App Canister not have enough Cycles, contact support"
            }
            ic_xrc_types::ExchangeRateError::InconsistentRatesReceived => "Inconsistent rate",
            ic_xrc_types::ExchangeRateError::Other(_) => "Cannot process, contact support",
        };

        return Err(mssg.to_string());
    } else {
        let rate = result.unwrap();
        let rate_num = rate.rate;
        // used 8 because ICP is 10^8
        let f_num =
            multiply_u64_with_power_of_10(rate_num, decimal as i32 - rate.metadata.decimals as i32);
        return Ok(f_num);
    }
}

fn multiply_u64_with_power_of_10(value: u64, power: i32) -> u64 {
    if power >= 0 {
        // Multiply by 10^power
        value.saturating_mul(10u64.saturating_pow(power as u32))
    } else {
        // Divide by 10^(-power)
        value / 10u64.saturating_pow((-power) as u32)
    }
}

pub fn process_metadata(metadata: Vec<(String, MetadataValue)>, info: &mut (String, usize, usize)) {
    for (key, value) in metadata {
        match key.as_str() {
            "icrc1:symbol" => {
                if let MetadataValue::Text(symbol) = value {
                    info.0 = symbol;
                }
            }
            "icrc1:decimals" => {
                if let MetadataValue::Nat(decimals) = value {
                    info.1 = decimals.0.try_into().unwrap()
                }
            }
            "icrc1:fee" => {
                if let MetadataValue::Nat(amount) = value {
                    info.2 = amount.0.try_into().unwrap()
                }
            }

            // Add more cases as needed
            _ => {
                continue;
            }
        }
    }
}

// fn extract_metadata(data : Vec<(String, MetadataValue)>) -> Metadata {
//     let mut meta = Metadata::default();
//     for (key, value) in data {
//         match key.as_str() {
//             "icrc1:symbol" => {
//                 if let MetadataValue::Text(symbol) = value {
//                     meta.symbol = symbol;
//                 }
//             }
//             "icrc1:decimals" => {
//                 if let MetadataValue::Nat(decimals) = value {
//                     meta.decimals = decimals.0.try_into().unwrap()
//                 }
//             }
//             "icrc1:fee" => {
//                 if let MetadataValue::Nat(amount) = value {
//                     meta.fee = amount.0.try_into().unwrap()
//                 }
//             }

//             // Add more cases as needed
//             _ => {
//                 continue;
//             }
//         }
//     }

//     meta
// }
pub async fn disburse_payment(
    token_principal: Principal,
    amt_to_transfer: u64,
    from_subaccount: Option<[u8; 32]>,
    to_account: Account
) -> Result<Nat, String>{

    let (reply, ) : (Nat,) = call(token_principal, "icrc1_fee", ()).await.unwrap();

    let fee:u64 = reply.0.try_into().unwrap();
    let actual_amt_to_transfer = amt_to_transfer - fee;
    let transfer_args = TransferArg {
        memo: None,
        amount: Nat::from(actual_amt_to_transfer),
        fee: None,
        from_subaccount,
        to: to_account,
        created_at_time: None,
    };

    let (resp,): (Result<Nat, TransferError>,) = call_with_payment(
        token_principal,
        "icrc1_transfer",
        (transfer_args,),
        20_000_000,
    )
    .await
    .unwrap();
    let nat_amount = resp.map_err(|err| err.to_string())?;
    Ok(nat_amount)

}

pub fn days_to_milliseconds(days: u64) -> u64 {
    const MILLISECONDS_PER_SECOND: u64 = 1_000;
    const SECONDS_PER_MINUTE: u64 = 60;
    const MINUTES_PER_HOUR: u64 = 60;
    const HOURS_PER_DAY: u64 = 24;

    days * HOURS_PER_DAY * MINUTES_PER_HOUR * SECONDS_PER_MINUTE * MILLISECONDS_PER_SECOND
}

pub async fn check_icrc_balance(token_canister_id: Principal, account_owner: Account) -> Result<u64, String> {
   

    match call_with_payment::<_, (Nat,)>(
        token_canister_id,
        "icrc1_balance_of",
        (account_owner,),
        0,
    )
    .await
    {
        Ok((balance,)) => Ok(balance.0.try_into().unwrap()),
        Err((_, err)) => Err(format!("Failed to check balance: {:?}", err)),
    }
}