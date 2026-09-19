use {
    velox_types::account::InstantiateMsg,
    bolt::{AuthCtx, MutableCtx, Response, Tx},
};

#[cfg_attr(not(feature = "library"), bolt::export)]
pub fn instantiate(ctx: MutableCtx, msg: InstantiateMsg) -> anyhow::Result<Response> {
    velox_auth::create_account(ctx, msg.activate)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), bolt::export)]
pub fn authenticate(ctx: AuthCtx, tx: Tx) -> anyhow::Result<Response> {
    velox_auth::authenticate_tx(ctx, tx, None)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), bolt::export)]
pub fn receive(ctx: MutableCtx) -> anyhow::Result<Response> {
    velox_auth::receive_transfer(ctx)?;

    Ok(Response::new())
}
