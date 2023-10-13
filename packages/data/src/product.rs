use crate::id;
use crate::Text;
use async_graphql::SimpleObject;

id!(ProductId);

#[derive(Clone, Copy, Serialize, SimpleObject)]
#[graphql(name = "BaseProduct")]
pub struct Product {
  pub id: ProductId,
  pub name: Text,
}
