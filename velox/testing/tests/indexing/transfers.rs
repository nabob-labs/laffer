use {
    assertor::*,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter},
    velox_app::Indexer,
    velox_primitives::{Addressable, Coins, Message, NonEmpty, ResultExt},
    velox_testing::{TestOption, setup_test_with_indexer},
    velox_types::{account_factory, constants::usdc},
};

#[tokio::test(flavor = "multi_thread")]
async fn index_transfer_events() -> anyhow::Result<()> {
    let (mut suite, mut accounts, _, contracts, _, velox_context, _, _, _db_guard) =
        setup_test_with_indexer(TestOption::default()).await;

    // Copied from benchmarks.rs
    let msgs = vec![
        Message::execute(
            contracts.account_factory,
            &account_factory::ExecuteMsg::RegisterAccount {},
            Coins::one(usdc::DENOM.clone(), 100_000_000).unwrap(),
        )
        .unwrap(),
    ];

    suite
        .send_messages_with_gas(
            &mut accounts.user1,
            50_000_000,
            NonEmpty::new_unchecked(msgs),
        )
        .await
        .should_succeed();

    suite.app.indexer.wait_for_finish().await?;

    // The 2 transfers should have been indexed.

    let blocks = velox_indexer_sql::entity::blocks::Entity::find()
        .all(&velox_context.db)
        .await?;

    assert_that!(blocks).has_length(1);

    let transfers = velox_indexer_sql::entity::transfers::Entity::find()
        .all(&velox_context.db)
        .await?;

    assert_that!(transfers).has_length(2);

    assert_that!(
        transfers
            .iter()
            .map(|t| t.amount.as_str())
            .collect::<Vec<_>>()
    )
    .is_equal_to(vec!["100000000", "100000000"]);

    let msg = Message::transfer(
        accounts.user1.address(),
        Coins::one(usdc::DENOM.clone(), 123).unwrap(),
    )
    .unwrap();

    suite
        .send_messages_with_gas(
            &mut accounts.user1,
            50_000_000,
            NonEmpty::new_unchecked(vec![msg]),
        )
        .await
        .should_succeed();

    // Force the runtime to wait for the async indexer task to finish
    suite.app.indexer.wait_for_finish().await?;

    // The transfer should have been indexed.
    let blocks = velox_indexer_sql::entity::blocks::Entity::find()
        .all(&velox_context.db)
        .await?;

    assert_that!(blocks).has_length(2);

    let transfers = velox_indexer_sql::entity::transfers::Entity::find()
        .filter(velox_indexer_sql::entity::transfers::Column::BlockHeight.eq(2))
        .all(&velox_context.db)
        .await?;

    assert_that!(transfers).has_length(1);

    assert_that!(
        transfers
            .iter()
            .map(|t| t.amount.as_str())
            .collect::<Vec<_>>()
    )
    .is_equal_to(vec!["123"]);

    Ok(())
}
