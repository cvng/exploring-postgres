use crate::database;
use crate::dispatch;
use crate::Error;
use crate::Result;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Show;
use bits_data::ShowCreated;
use bits_data::ShowId;
use bits_data::UserId;

#[derive(InputObject)]
pub struct CreateShowInput {
  pub creator_id: UserId,
}

#[derive(SimpleObject)]
pub struct CreateShowPayload {
  pub show: Show,
}

pub async fn create_show(input: CreateShowInput) -> Result<CreateShowPayload> {
  let show = Show {
    id: ShowId::new(),
    creator_id: input.creator_id,
  };

  dispatch::dispatch(vec![ShowCreated { show: show.clone() }.into()])?;

  Ok(CreateShowPayload {
    show: database::db()
      .shows
      .get(&show.id)
      .ok_or_else(|| Error::NotFound(show.id.to_string()))?
      .clone(),
  })
}
