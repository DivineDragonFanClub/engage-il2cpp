
#[cfg(any(feature = "system-io-memorystream-types"))]
pub mod memorystream;
#[cfg(any(feature = "system-io-memorystream-types"))]
pub use memorystream::*;
#[cfg(any(feature = "system-io-stream-types"))]
pub mod stream;
#[cfg(any(feature = "system-io-stream-types"))]
pub use stream::*;
