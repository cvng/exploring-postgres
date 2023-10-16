use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Comment;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;
use thiserror::Error;

#[derive(Copy, Clone, Serialize, InputObject)]
pub struct CommentInput {
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}

#[derive(Serialize, SimpleObject)]
pub struct CommentPayload {
  pub comment: Comment,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
}

#[derive(Default)]
struct CommentCommand {
  show: Option<Show>,
}

impl CommentCommand {
  pub fn new(show: Option<Show>) -> Self {
    Self { show }
  }
}

impl Command for CommentCommand {
  type Error = Error;
  type Event = Event;
  type Input = CommentInput;
  type State = CommentCommand;
  type Payload = CommentPayload;

  fn handle(
    state: &Self::State,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    state.show.ok_or(Error::ShowNotFound(input.show_id))?;

    let comment = Comment {
      id: CommentId::new(),
      user_id: input.user_id,
      show_id: input.show_id,
      text: input.text,
    };

    Ok(vec![Event::comment_created(comment)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Payload> {
    events.iter().fold(None, |_, event| match event {
      Event::CommentCreated { payload } => Some(CommentPayload {
        comment: payload.comment,
      }),
      _ => None,
    })
  }
}

pub fn comment(input: CommentInput) -> Result<CommentPayload, Error> {
  let show = database::db().shows.get(&input.show_id).cloned();

  let state = CommentCommand::new(show);

  CommentCommand::handle(&state, input)
    .map(|events| dispatcher::dispatch(events).unwrap())
    .map(|events| CommentCommand::apply(events).unwrap())
}

#[test]
fn test_comment() {
  let state = CommentCommand {
    show: Some(bits_data::Show {
      id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      creator_id: UserId::new(),
      name: Text::new("name"),
      started_at: None,
    }),
  };

  let input = CommentInput {
    user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: state.show.unwrap().id,
    text: Text::new("text"),
  };

  let events = CommentCommand::handle(&state, input).unwrap();

  assert_json_snapshot!(events, {
    "[0].payload.comment.id" => "[uuid]",
  }, @r###"
  [
    {
      "type": "comment_created",
      "payload": {
        "comment": {
          "id": "[uuid]",
          "user_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
          "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
          "text": "text"
        }
      }
    }
  ]
  "###);
}
