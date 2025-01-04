use thiserror::Error;

pub type Result<T, E=RpgError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum RpgError {
    #[error("could not convert {0:?}")]
    PointConversion(#[from] std::num::TryFromIntError),

    #[error("out of map bounds")]
    OutOfBounds,

    #[error("unexpectedly empty: {0}")]
    Empty(String),

    #[error("error from eframe {0:?}")]
    Eframe(#[from] eframe::Error),
}
