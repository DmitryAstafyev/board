use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Entity {0} doesn't have parent")]
    NoParent(String),
    // #[error("Access error: {0}")]
    // AccessError(String),
    // #[error("Invalid configuration: {0}")]
    // InvalidConfiguration(String),
    // #[error("Toml error")]
    // PasringToml(#[from] toml::de::Error),
    #[error("Render isn't inited")]
    RenderNotInited,
    #[error("Static error message")]
    NotSupported,
    #[error("{0}")]
    Other(String),
}
