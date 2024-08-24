//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "disc")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub album_db_id: i32,
    pub index: i32,
    pub title: Option<String>,
    pub catalog: Option<String>,
    pub artist: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::album::Entity",
        from = "Column::AlbumDbId",
        to = "super::album::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Album,
    #[sea_orm(has_many = "super::track::Entity")]
    Track,
}

impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def()
    }
}

impl Related<super::track::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Track.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
