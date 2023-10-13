use crate::id;
use crate::BidId;
use crate::DateTime;
use crate::ProductId;
use crate::ShowId;
use async_graphql::SimpleObject;

id!(AuctionId);
id!(AuctionProductId);

#[derive(Clone, Copy, SimpleObject)]
#[graphql(name = "BaseAuction")]
pub struct Auction {
  pub id: AuctionId,
  pub show_id: ShowId,
  pub ready_at: Option<DateTime>,
  pub started_at: Option<DateTime>,
  pub expired_at: Option<DateTime>,
}

#[derive(Clone, Serialize, SimpleObject)]
pub struct AuctionProduct {
  pub id: AuctionProductId,
  pub auction_id: AuctionId,
  pub product_id: ProductId,
  pub best_bid_id: Option<BidId>,
}
