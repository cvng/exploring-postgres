//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "show")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub created: DateTime,
  pub updated: Option<DateTime>,
  pub creator_id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  pub started: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
  #[sea_orm(has_many = "super::auction::Entity")]
  Auction,
  #[sea_orm(has_many = "super::comment::Entity")]
  Comment,
  #[sea_orm(
    belongs_to = "super::person::Entity",
    from = "Column::CreatorId",
    to = "super::person::Column::Id",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  Person,
}

impl Related<super::auction::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Auction.def()
  }
}

impl Related<super::comment::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Comment.def()
  }
}

impl Related<super::person::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Person.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, DeriveRelatedEntity, EnumIter)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::auction::Entity")]
  Auction,
  #[sea_orm(entity = "super::comment::Entity")]
  Comment,
  #[sea_orm(entity = "super::person::Entity")]
  Person,
}
