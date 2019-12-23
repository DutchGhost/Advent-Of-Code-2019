//! I/O
//! 
//! This module contains the [`Read`] and [`Write`] traits.
//! 
//! Furthermore it contains several implementations of these
//! traits, which can be combined in various ways.
use crate::device::Poll;

mod socket;
pub use socket::Socket;

pub mod read;
pub mod write;

#[doc(inline)]
pub use read::Read;

#[doc(inline)]
pub use write::Write;