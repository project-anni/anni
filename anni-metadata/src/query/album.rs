use crate::{schema, DateTime, Json, Uuid};

#[derive(cynic::QueryVariables, Debug)]
pub struct AlbumVariables {
    pub album_id: Uuid,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MetadataQuery", variables = "AlbumVariables")]
pub struct AlbumQuery {
    #[arguments(albumId: $album_id)]
    pub album: Option<Album>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Album")]
pub struct Album {
    pub id: cynic::Id,
    pub album_id: Uuid,
    pub level: MetadataOrganizeLevel,
    pub title: String,
    pub edition: Option<String>,
    pub catalog: Option<String>,
    pub artist: String,
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub extra: Option<Json>,
    pub discs: Vec<Disc>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Disc {
    pub id: cynic::Id,
    pub index: i32,
    pub title: Option<String>,
    pub catalog: Option<String>,
    pub artist: Option<String>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub tracks: Vec<Track>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Track {
    pub id: cynic::Id,
    pub index: i32,
    pub title: String,
    pub artist: String,
    #[cynic(rename = "type")]
    pub type_: TrackType,
    pub artists: Option<Json>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Tag {
    pub id: cynic::Id,
    pub name: String,
    #[cynic(rename = "type")]
    pub type_: TagType,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum MetadataOrganizeLevel {
    Initial,
    Partial,
    Reviewed,
    Finished,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum TrackType {
    Normal,
    Instrumental,
    Absolute,
    Drama,
    Radio,
    Vocal,
    Unknown,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum TagType {
    Artist,
    Group,
    Animation,
    Radio,
    Series,
    Project,
    Game,
    Organization,
    Others,
}
