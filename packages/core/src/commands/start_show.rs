use crate::database;
use crate::dispatch;
use crate::Error;
use crate::Result;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::ShowStarted;
use bits_data::Utc;

#[derive(InputObject)]
pub struct StartShowInput {
  pub id: ShowId,
}

#[derive(SimpleObject)]
pub struct StartShowPayload {
  pub show: Show,
}

pub async fn start_show(input: StartShowInput) -> Result<StartShowPayload> {
  let show = database::db()
    .shows
    .get(&input.id)
    .cloned()
    .ok_or_else(|| Error::NotFound(input.id.into()))?;

  if show.started_at.is_some() {
    return Err(Error::AlreadyStarted(input.id.into()));
  }

  dispatch::dispatch(vec![ShowStarted {
    id: input.id,
    started_at: Utc::now(),
  }
  .into()])?;

  Ok(StartShowPayload {
    show: database::db()
      .shows
      .get(&input.id)
      .cloned()
      .ok_or_else(|| Error::NotFound(input.id.into()))?,
  })
}
