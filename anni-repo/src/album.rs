use serde::{Serialize, Deserialize, Deserializer, Serializer};
use std::str::FromStr;
use std::path::Path;
use crate::Datetime;
use anni_common::traits::FromFile;
use anni_derive::FromFile;
use anni_common::inherit::InheritableValue;

#[derive(Serialize, Deserialize, FromFile)]
pub struct Album {
    #[serde(rename = "album")]
    info: AlbumInfo,
    discs: Vec<Disc>,
}

impl Album {
    pub fn new(title: String, artist: String, release_date: Datetime, catalog: String) -> Self {
        Album {
            info: AlbumInfo {
                title: InheritableValue::own(title),
                artist: InheritableValue::own(artist),
                release_date,
                album_type: TrackType::Normal,
                catalog,
            },
            discs: Vec::new(),
        }
    }
}

impl FromStr for Album {
    type Err = crate::Error;

    fn from_str(toml_str: &str) -> Result<Self, Self::Err> {
        let mut album: Album = toml::from_str(toml_str)
            .map_err(|e| crate::Error::TomlParseError {
                target: "Album",
                err: e,
            })?;

        album.inherit();
        Ok(album)
    }
}

impl ToString for Album {
    fn to_string(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}

impl Album {
    pub fn title(&self) -> &str {
        self.info.title.as_ref()
    }

    pub fn artist(&self) -> &str {
        self.info.artist.as_ref()
    }

    pub fn release_date(&self) -> &Datetime {
        &self.info.release_date
    }

    pub fn track_type(&self) -> TrackType {
        self.info.album_type.clone()
    }

    pub fn catalog(&self) -> &str {
        self.info.catalog.as_ref()
    }

    pub fn discs(&self) -> &Vec<Disc> {
        &self.discs
    }

    pub fn inherit(&mut self) {
        for disc in self.discs.iter_mut() {
            disc.title.inherit_from(&self.info.title);
            disc.artist.inherit_from(&self.info.artist);
            disc.disc_type.inherit_from_owned(&self.info.album_type);
            disc.inherit();
        }
    }

    pub fn add_disc(&mut self, mut disc: Disc) {
        disc.title.inherit_from(&self.info.title);
        disc.artist.inherit_from(&self.info.artist);
        disc.disc_type.inherit_from_owned(&self.info.album_type);
        self.discs.push(disc);
    }
}

#[derive(Serialize, Deserialize)]
struct AlbumInfo {
    title: InheritableValue<String>,
    artist: InheritableValue<String>,
    #[serde(rename = "date")]
    release_date: Datetime,
    #[serde(rename = "type")]
    album_type: TrackType,
    catalog: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Disc {
    catalog: String,
    title: InheritableValue<String>,
    artist: InheritableValue<String>,
    #[serde(rename = "type")]
    disc_type: InheritableValue<TrackType>,
    tracks: Vec<Track>,
}

impl Disc {
    pub fn new(catalog: String, title: InheritableValue<String>, artist: InheritableValue<String>, disc_type: InheritableValue<TrackType>) -> Self {
        Disc {
            catalog,
            title,
            artist,
            disc_type,
            tracks: Vec::new(),
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn artist(&self) -> &str {
        self.artist.as_ref()
    }

    pub fn catalog(&self) -> &str {
        self.catalog.as_ref()
    }

    pub fn tracks(&self) -> &Vec<Track> {
        self.tracks.as_ref()
    }

    pub fn inherit(&mut self) {
        for track in self.tracks.iter_mut() {
            track.artist.inherit_from(&self.artist);
            track.track_type.inherit_from(&self.disc_type);
        }
    }

    pub fn add_track(&mut self, mut track: Track) {
        track.artist.inherit_from(&self.artist);
        track.track_type.inherit_from(&self.disc_type);
        self.tracks.push(track);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    title: String,
    artist: InheritableValue<String>,
    #[serde(rename = "type")]
    track_type: InheritableValue<TrackType>,
}

impl Track {
    pub fn new(title: String, artist: InheritableValue<String>, track_type: InheritableValue<TrackType>) -> Self {
        Track {
            title,
            artist,
            track_type,
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn artist(&self) -> &str {
        self.artist.as_ref()
    }

    pub fn track_type(&self) -> TrackType {
        self.track_type.get_raw()
    }
}

#[derive(Clone, Debug)]
pub enum TrackType {
    Normal,
    Instrumental,
    Absolute,
    Drama,
    Radio,
    Other(String),
}

impl AsRef<str> for TrackType {
    fn as_ref(&self) -> &str {
        match &self {
            TrackType::Normal => "normal",
            TrackType::Instrumental => "instrumental",
            TrackType::Absolute => "absolute",
            TrackType::Drama => "drama",
            TrackType::Radio => "radio",
            TrackType::Other(s) => s.as_ref(),
        }
    }
}

impl Serialize for TrackType {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for TrackType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "normal" => TrackType::Normal,
            "instrumental" => TrackType::Instrumental,
            "absolute" => TrackType::Absolute,
            "drama" => TrackType::Drama,
            "radio" => TrackType::Radio,
            _ => TrackType::Other(s),
        })
    }
}
