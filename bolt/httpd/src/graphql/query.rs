use {async_graphql::MergedObject, bolt::BoltQuery};

pub mod bolt;

#[derive(MergedObject, Default)]
pub struct Query(BoltQuery);
