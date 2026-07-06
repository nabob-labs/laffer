use {
    crate::{
        maker_priority_handler::MakerPriorityHandler,
        pyth_handler::{PythHandler, QueryPythId},
    },
    prost::bytes::Bytes,
    reqwest::IntoUrl,
    std::fmt::Debug,
    velox_primitives::{Lengthy, NonEmpty, QuerierWrapper, StdError},
    velox_pyth_client::{PythClient, PythClientCache, PythClientTrait},
    velox_pyth_types::constants::LAZER_ENDPOINTS_TEST,
};

pub struct ProposalPreparer<P>
where
    P: PythClientTrait + QueryPythId,
{
    maker_priority: MakerPriorityHandler,
    pyth: PythHandler<P>,
}

impl<P> Clone for ProposalPreparer<P>
where
    P: PythClientTrait + QueryPythId,
{
    fn clone(&self) -> Self {
        Self {
            maker_priority: self.maker_priority,
            pyth: self.pyth.clone(),
        }
    }
}

impl ProposalPreparer<PythClient> {
    pub fn new<V, U, T>(endpoints: V, access_token: T) -> Self
    where
        V: IntoIterator<Item = U> + Lengthy,
        U: IntoUrl,
        T: ToString,
    {
        Self {
            maker_priority: MakerPriorityHandler,
            pyth: PythHandler::new(endpoints, access_token),
        }
    }
}

impl ProposalPreparer<PythClientCache> {
    pub fn new_with_cache() -> Self {
        // `LAZER_ENDPOINTS_TEST` is a static non-empty array.
        Self {
            maker_priority: MakerPriorityHandler,
            pyth: PythHandler::new_with_cache(
                NonEmpty::new_unchecked(LAZER_ENDPOINTS_TEST),
                "lazer_token",
            ),
        }
    }
}

impl<P> velox_app::ProposalPreparer for ProposalPreparer<P>
where
    P: PythClientTrait + QueryPythId + Send + 'static,
    P::Error: Debug,
{
    type Error = StdError;

    fn prepare_proposal(
        &self,
        querier: QuerierWrapper,
        txs: Vec<Bytes>,
        max_tx_bytes: usize,
    ) -> Result<Vec<Bytes>, Self::Error> {
        // Maker-priority promotion runs first; Pyth runs second so the
        // oracle-update tx ends up at index 0, ahead of the priority maker
        // traffic. Final layout: `[oracle, priority_makers..., others...]`.
        let txs = self
            .maker_priority
            .prepare_proposal(querier, txs, max_tx_bytes)?;
        self.pyth.prepare_proposal(querier, txs, max_tx_bytes)
    }
}
