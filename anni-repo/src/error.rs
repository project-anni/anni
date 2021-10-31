use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid {target} toml: {err:?}\n{input}")]
    TomlParseError {
        target: &'static str,
        input: String,
        err: toml::de::Error,
    },

    #[error("failed to initialize repository: {0}")]
    RepoInitError(anyhow::Error),

    #[error("failed to load album {album:?} in repository: {err:?}")]
    RepoAlbumLoadError { album: String, err: anyhow::Error },

    #[error("failed to load tags from {file:?}: {err:?}")]
    RepoTagLoadError { file: std::path::PathBuf, err: anyhow::Error },

    #[error("duplicated tag {0}")]
    RepoTagDuplicate(crate::tag::TagRef),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
