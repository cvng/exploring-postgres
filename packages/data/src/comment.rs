use crate::id;
use crate::DateTime;
use crate::ShowId;
use crate::Text;
use crate::UserId;
use async_graphql::SimpleObject;

id!(CommentId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
pub struct Comment {
  pub id: CommentId,
  pub created: Option<DateTime>,
  pub updated: Option<DateTime>,
  pub author_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}
