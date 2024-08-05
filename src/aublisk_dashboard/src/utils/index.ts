import { APP_CANISTER_ID, HOST } from "@/config";
import { IcrcLedgerCanister, IcrcTokenMetadataResponse, IcrcAccount } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import { createAgent, principalToSubAccount } from "@dfinity/utils";
import { TokenInfo } from "./types";

export async function get_icrc_info(address : string) {
    let agent = await createAgent({
        host: HOST,
        //@ts-ignore
        identity: null
    })
    let ledger = IcrcLedgerCanister.create({
        agent,
        canisterId: Principal.fromText(address)
    })

    return await ledger.metadata({})

}

export async function hexToUint8Array(hexString : string) {
    if (hexString.length % 2 !== 0) {
      throw new Error('Invalid hex string');
    }
    const arrayBuffer = new Uint8Array(hexString.length / 2);
  
    for (let i = 0; i < hexString.length; i += 2) {
      const byteValue = parseInt(hexString.substr(i, 2), 16);
      if (isNaN(byteValue)) {
        throw new Error('Invalid hex string');
      }
      arrayBuffer[i / 2] = byteValue;
    }
  
    return arrayBuffer;
  }


export async function getTokenInfo(canisterId: string): Promise<TokenInfo> {
  try {
    const agent = await createAgent({
      host: HOST,
      //@ts-ignore
      identity: undefined
    });

    const canister = IcrcLedgerCanister.create({
      agent,
      canisterId: Principal.fromText(canisterId),
    });

    const metadata: IcrcTokenMetadataResponse = await canister.metadata({});

    const getName = (metadata: IcrcTokenMetadataResponse) =>
        //@ts-ignore
      metadata.find(([key]) => key === "icrc1:name")?.[1]?.Text || "";

    const getSymbol = (metadata: IcrcTokenMetadataResponse) =>
        //@ts-ignore
      metadata.find(([key]) => key === "icrc1:symbol")?.[1]?.Text || "";

    const getDecimals = (metadata: IcrcTokenMetadataResponse) =>
      Number(
         //@ts-ignore
        metadata.find(([key]) => key === "icrc1:decimals")?.[1]?.Nat || "0"
      );

    const getLogo = (metadata: IcrcTokenMetadataResponse) =>
         //@ts-ignore
      metadata.find(([key]) => key === "icrc1:logo")?.[1]?.Text || "";

    return {
      name: getName(metadata),
      canister_id: canisterId,
      logo_url: getLogo(metadata),
      decimals: getDecimals(metadata),
      symbol: getSymbol(metadata)
    };
  } catch (error) {
    console.error("Error fetching token info:", error);
    throw error;
  }
}

export function clipString(str : string, maxLength = 100) {
  if (str.length <= maxLength) {
    return str;
  }
  return str.slice(0, maxLength - 3) + '...';
}

export async function get_wallet_balance(principal : Principal, icrc_token_id : string) {
  let account : IcrcAccount = {owner : Principal.fromText(APP_CANISTER_ID), subaccount: principalToSubAccount(principal)}

  const agent = await createAgent({
    host: HOST,
    //@ts-ignore
    identity: undefined
  });
  let ledger_canister = IcrcLedgerCanister.create({canisterId: Principal.fromText(icrc_token_id), agent})
  let amount_bigint = await ledger_canister.balance(account)
  return Number(amount_bigint)
}