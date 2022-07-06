use std::fmt;

#[derive(Debug)]
pub enum Error {
    WrongNumberOfChunks,
    InputSmallerThanNumberOfChunks,
    RandBytesFailure,
    ChunkParseError,
    IOError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WrongNumberOfChunks => {
                write!(f, "WrongNumberOfChunks: Could not combine chunks because too few or many chunks were found")
            }
            Self::InputSmallerThanNumberOfChunks => {
                write!(f, "The provided input file about to be split is smaller than the number of chunks to be split into")
            }
            Self::RandBytesFailure => {
                write!(f, "Failed to generate random key")
            }
            Self::ChunkParseError => {
                write!(f, "Failed to parse chunk")
            }
            Self::IOError(e) => {
                write!(f, "IO Error: {}", e)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
