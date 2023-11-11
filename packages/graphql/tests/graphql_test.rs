mod common;

use crate::common::execute;
use crate::common::Context;
use crate::common::TestToken;
use graphql_client::GraphQLQuery;
#[cfg(test)]
use insta::assert_json_snapshot;
use test_context::test_context;
use tokio::test;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "tests/operations.graphql"
)]
pub struct BidMutation;

#[test_context(Context)]
#[test]
async fn test_bid_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    BidMutation::build_query(bid_mutation::Variables {}),
    Some(TestToken::buyer_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.bid.bid.id" => "[uuid]" });
}

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "tests/operations.graphql"
)]
pub struct StartMutation;

#[test_context(Context)]
#[test]
async fn test_start_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    StartMutation::build_query(start_mutation::Variables {}),
    Some(TestToken::seller_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.start.auction.id" => "[uuid]" });
}
