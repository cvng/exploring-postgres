use crate::database;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::AuctionId;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Utc;
use thiserror::Error;

#[derive(InputObject)]
pub struct StartShowInput {
  pub id: ShowId,
}

#[derive(SimpleObject)]
pub struct StartShowResult {
  pub show: Show,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(ShowId),
  #[error("already started: {0}")]
  AlreadyStarted(ShowId),
  #[error("auction not found: {0}")]
  AuctionNotFound(ShowId),
  #[error("auction not ready: {0}")]
  AuctionNotReady(AuctionId),
}

pub async fn start_show(
  input: StartShowInput,
) -> Result<StartShowResult, Error> {
  let show = database::db()
    .shows
    .get(&input.id)
    .cloned()
    .ok_or(Error::NotFound(input.id))?;

  // Check that the show hasn't already started.
  if show.started_at.is_some() {
    return Err(Error::AlreadyStarted(show.id));
  }

  let auction = database::db()
    .auctions
    .values()
    .find(|auction| auction.show_id == show.id)
    .cloned()
    .ok_or(Error::AuctionNotFound(show.id))?;

  // Check that the auction is ready.
  if auction.ready_at.is_none() {
    return Err(Error::AuctionNotReady(auction.id));
  }

  let now = Utc::now();

  dispatcher::dispatch(vec![
    Event::show_started(show.id, now),
    Event::auction_started(auction.id, now),
  ])
  .ok();

  Ok(StartShowResult { show })
}
