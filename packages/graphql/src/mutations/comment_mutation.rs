use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::comment::CommentInput;
use bits_core::comment::CommentResult;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;

pub struct CommentMutation;

impl CommentMutation {
  pub fn type_name() -> &'static str {
    "comment"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(CommentResult::type_name()),
      |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<CommentInput>()?;

          let result = commands::comment::comment(&client, input).await?;

          Ok(Some(to_value(result)?))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(CommentInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder.inputs.push(CommentInput::to_input());
  builder.outputs.push(CommentResult::to_object());
  builder.mutations.push(CommentMutation::to_field());
  builder
}
