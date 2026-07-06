use {
    async_trait::async_trait,
    std::{fmt::Display, pin::Pin},
    velox_primitives::{Lengthy, NonEmpty},
    velox_pyth_types::{PriceUpdate, PythLazerSubscriptionDetails},
};

#[async_trait]
pub trait PythClientTrait: Clone {
    type Error: Display;

    /// Creates a stream of price updates for the given ids.
    async fn stream<I>(
        &mut self,
        ids: NonEmpty<I>,
    ) -> Result<Pin<Box<dyn tokio_stream::Stream<Item = PriceUpdate> + Send>>, Self::Error>
    where
        I: IntoIterator<Item = PythLazerSubscriptionDetails> + Lengthy + Send + Clone;

    /// Closes the stream.
    fn close(&mut self);
}
