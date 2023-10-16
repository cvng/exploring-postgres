use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use bits_core::commands;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
  async fn bid(
    &self,
    _ctx: &Context<'_>,
    input: commands::bid::BidInput,
  ) -> Result<commands::bid::BidResult> {
    Ok(commands::bid::bid(input)?)
  }

  async fn comment(
    &self,
    _ctx: &Context<'_>,
    input: commands::comment::CommentInput,
  ) -> Result<commands::comment::CommentResult> {
    Ok(commands::comment::comment(input)?)
  }

  async fn create_product(
    &self,
    _ctx: &Context<'_>,
    input: commands::create_product::CreateProductInput,
  ) -> Result<commands::create_product::CreateProductResult> {
    Ok(commands::create_product::create_product(input).await?)
  }

  async fn create_show(
    &self,
    _ctx: &Context<'_>,
    input: commands::create_show::CreateShowInput,
  ) -> Result<commands::create_show::CreateShowResult> {
    Ok(commands::create_show::create_show(input).await?)
  }

  async fn start_show(
    &self,
    _ctx: &Context<'_>,
    input: commands::start_show::StartShowInput,
  ) -> Result<commands::start_show::StartShowResult> {
    Ok(commands::start_show::start_show(input).await?)
  }

  async fn add_auction_product(
    &self,
    _ctx: &Context<'_>,
    input: commands::add_auction_product::AddAuctionProductInput,
  ) -> Result<commands::add_auction_product::AddAuctionProductResult> {
    Ok(commands::add_auction_product::add_auction_product(input).await?)
  }
}
