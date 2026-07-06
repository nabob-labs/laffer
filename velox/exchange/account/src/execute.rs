use {
    velox_primitives::{AuthCtx, MutableCtx, Response, Tx},
    velox_types::account::InstantiateMsg,
};

pub fn instantiate(ctx: MutableCtx, msg: InstantiateMsg) -> anyhow::Result<Response> {
    velox_auth::create_account(ctx, msg.activate)?;

    Ok(Response::new())
}

pub fn authenticate(ctx: AuthCtx, tx: Tx) -> anyhow::Result<Response> {
    velox_auth::authenticate_tx(ctx, tx, None)?;

    Ok(Response::new())
}

pub fn receive(ctx: MutableCtx) -> anyhow::Result<Response> {
    velox_auth::receive_transfer(ctx)?;

    Ok(Response::new())
}
