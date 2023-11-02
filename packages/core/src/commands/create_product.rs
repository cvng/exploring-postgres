use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::UserId;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductInput {
  pub creator_id: UserId,
  pub name: String,
}

impl CreateProductInput {
  pub fn type_name() -> &'static str {
    "CreateProductInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
  }
}

#[derive(Serialize)]
pub struct CreateProductResult {
  pub product: Product,
}

impl CreateProductResult {
  pub fn type_name() -> &'static str {
    "CreateProductResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "id".to_string(),
      TypeRef::named_nn(TypeRef::ID),
      |ctx| {
        FieldFuture::new(
          async move { Ok(ctx.parent_value.as_value().cloned()) },
        )
      },
    ))
  }
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("product not created")]
  NotCreated,
}

pub struct CreateProductCommand {}

impl Command for CreateProductCommand {
  type Error = Error;
  type Event = Event;
  type Input = CreateProductInput;
  type Result = CreateProductResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![Event::product_created(
      ProductId::new_v4(),
      input.creator_id,
      input.name,
    )])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ProductCreated { data, .. } => Some(CreateProductResult {
        product: Product {
          id: data.id,
          created: None,
          updated: None,
          creator_id: data.creator_id,
          name: data.name.clone(),
        },
      }),
      _ => None,
    })
  }
}

pub async fn create_product(
  client: &Client,
  input: CreateProductInput,
) -> Result<CreateProductResult, Error> {
  dispatcher::dispatch(client, CreateProductCommand {}.handle(input)?)
    .await
    .map(CreateProductCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_create_product() {
  let input = CreateProductInput {
    creator_id: "abbba031-f122-42b8-b6ff-585ad245aadd".parse().unwrap(),
    name: "name".parse().unwrap(),
  };

  let events = CreateProductCommand {}.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "product_created",
      "data": {
        "id": "49a3c0eb-0bd6-4954-b464-7f75b2f18b44",
        "creator_id": "abbba031-f122-42b8-b6ff-585ad245aadd",
        "name": "name"
      }
    }
  ]
  "###);
}
