// cuadra::error

/// An error that can arise during the use of a grid.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The given indices were out of bounds.
    // TODO: more dimensions
    IndicesOutOfBounds(usize, usize),

    /// The given index in row or column major order was out of bounds.
    IndexOutOfBounds(usize),

    /// The given indices were out of bounds for a chunk of the given length.
    ChunkIndicesOutOfBounds(usize, usize, usize),

    /// The dimensions given did not match the elements provided
    DimensionMismatch,

    /// There were not enough elements to fill the array.
    NotEnoughElements,
}

/// A result that already includes [`Error`].
pub type Result<T> = core::result::Result<T, Error>;
