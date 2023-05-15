use cosmwasm_std::{Api, Binary, Coin, Env, Extern, HandleResponse, HumanAddr, Querier, StdError, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use secret_toolkit::snip20::{ExecuteMsg as Snip20ExecuteMsg, Snip20ReceiveMsg};

//Define NFT Struct 
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct NFT {
    pub id: String,
    pub owner: HumanAddr,
    pub token_uri: String, 
    pub lp_tokens: Vec<ShadeLPToken>,
}

//Define LP Token struct
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ShadeLPToken {
    pub address: HumanAddr, 
    pub amount: Coin,
}

//Define the message to wrap LP Tokens into the NFT
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct WrapLPIntoNFTMsg {
    pub id: String,
    pub lp_tokens: Vec<ShadeLPToken>,
}

//Define the function to wrap LP Tokens into NFT
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub fn wrap_lp_into_nft (
    deps: &mut Extern<SecretStorage, RunTime>,
    env:Env, 
    info: MessageInfo,
    msg:WrapLPIntoNFTMsg,
) -> Result <HandleResponse, StdError> {
    //Get the NFT from Storage 
    let mut nft_storage = singleton_mut::<&mut Storage, &str, NFT>(&mut deps.storage, msg.id.as_str());
    let nft = nft_storage

    // Get the Shade Protocol contract address from env
    let shade_protocol_addr = env.contract.address.clone();
    // Iterate over the LP tokens
    for lp_token in msg.lp_tokens {
        // Transfer LP tokens from the sender to the NFT contract
        let transfer_msg = Snip20ExecuteMsg::TransferFrom {
            owner: info.sender.clone(),
            recipient: shade_protocol_addr.clone(),
            amount: lp_token.amount.clone(),
            memo: "Wrap LP into NFT".to_string(),
        };
        let transfer_response: HandleResponse = snip20::execute(deps, env.clone(), transfer_msg.into())?;

        // Add the LP tokens to the NFT
        nft.lp_tokens.push(lp_token);
    }

    // Save the updated NFT to storage
    nft_storage.save(&nft)?;

    // Return success response
    Ok(HandleResponse::default())
}
//Unwrap LP Tokens from NFT to Shade
pub fn unwrap_lp_tokens(&self, env: Env, token_id: TokenId, amount: Uint128) -> ContractResult<Response> {
    // get the owner of the NFT
    let sender = env.message.sender;
    let owner_raw = self.nft_minter.owner_of(env.clone(), token_id)?;
    let owner = owner_raw.as_str();
    assert_eq!(sender, owner, "Only the NFT owner can unwrap LP tokens");

    // get the LP token address and balance of the NFT
    let lp_token_address = self.get_lp_token_address(env.clone(), token_id)?;
    let nft_balance = self.get_lp_token_balance(env.clone(), token_id)?;

    // calculate the LP token amount to be transferred
    let transfer_amount = amount.u128().checked_mul(nft_balance.u128()).unwrap().checked_div(self.nft_total_supply(env.clone(), token_id).u128()).unwrap().into();

    // transfer the LP tokens from the contract to the owner
    let msg = Cw20HandleMsg::Transfer {
        recipient: sender.into(),
        amount: transfer_amount,
    };
    let res = self.lp_token_contract.execute(env.clone(), lp_token_address.into(), &msg.into())?;
    Ok(res.into())
}