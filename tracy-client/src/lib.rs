//! This crate is a set of safe bindings to the client library of the [Tracy profiler].
//!
//! If you have already instrumented your application with `tracing`, consider `tracing-tracy`.
//!
//! # Important note
//!
//! Simply depending on this crate is sufficient for tracy to be enabled at program startup, even
//! if none of the APIs provided by this crate are invoked. Tracy will broadcast discovery packets
//! to the local network and expose the data it collects in the background to that same network.
//! Traces collected by Tracy may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client` crate conditionally, via the
//! `enable` feature flag provided by this crate.
//!
//! [Tracy profiler]: https://github.com/wolfpld/tracy
#![feature(const_mut_refs, const_type_name)]

#[cfg(feature = "enable")]
mod enabled;

#[cfg(feature = "enable")]
#[doc(hidden)]
pub use const_format as cf;

#[cfg(feature = "enable")]
#[doc(hidden)]
pub use tracy_client_sys as sys;

#[cfg(feature = "enable")]
pub use self::enabled::{
    Span,
    ProfiledAllocator,
    Frame,
    message,
    color_message,
    set_thread_name,
    Plot,
};

#[cfg(not(feature = "enable"))]
mod disabled;

#[cfg(not(feature = "enable"))]
pub use self::disabled::{
    Span,
    ProfiledAllocator,
    message,
    color_message,
    set_thread_name,
    Plot,
};
