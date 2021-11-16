use std::alloc;

#[macro_export]
macro_rules! static_span {
    () => {{
    }};
    ($name:expr) => {{
    }};
}

pub struct Span;

impl Span {
    /// Start a new Tracy span.
    ///
    /// This function allocates the span information on the heap until it is read out by the
    /// profiler.
    ///
    /// `callstack_depth` specifies the maximum number of stack frames client should collect.
    pub fn new(_name: &str, _function: &str, _file: &str, _line: u32, _callstack_depth: u16) -> Self {
        Self
    }

    /// Emit a numeric value associated with this span.
    pub fn emit_value(&self, _value: u64) {
    }

    /// Emit some text associated with this span.
    pub fn emit_text(&self, _text: &str) {
    }
}

/// A profiling wrapper around an allocator.
///
/// See documentation for [`std::alloc`](std::alloc) for more information about global allocators.
///
/// # Examples
///
/// In your executable, add:
///
/// ```rust
/// # use tracy_client::*;
/// #[global_allocator]
/// static GLOBAL: ProfiledAllocator<std::alloc::System> =
///     ProfiledAllocator::new(std::alloc::System, 100);
/// ```
pub struct ProfiledAllocator<T>(T);

impl<T> ProfiledAllocator<T> {
    pub const fn new(inner_allocator: T, _callstack_depth: u16) -> Self {
        Self(inner_allocator)
    }
}

unsafe impl<T: alloc::GlobalAlloc> alloc::GlobalAlloc for ProfiledAllocator<T> {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        self.0.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: alloc::Layout) -> *mut u8 {
        self.0.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: alloc::Layout, new_size: usize) -> *mut u8 {
        self.0.realloc(ptr, layout, new_size)
    }
}

/// Indicate that rendering of a continuous frame has ended.
///
/// Typically should be inserted after a buffer swap.
///
/// In case you want to annotate secondary continuous frame sets, call the macro with a string
/// argument.
///
/// For non-continuous frame sets see [`Frame`](Frame).
///
/// # Examples
///
/// ```no_run
/// # use tracy_client::*;
/// # fn swap_buffers() {}
/// swap_buffers();
/// finish_continuous_frame!();
/// finish_continuous_frame!("some other frame loop");
/// ```
#[macro_export]
macro_rules! finish_continuous_frame {
    () => {
        {}
    };
    ($name: literal) => {
        {}
    };
}

/// Start a non-continuous frame region.
#[macro_export]
macro_rules! start_noncontinuous_frame {
    ($name: literal) => {
        ()
    };
}


/// Output a message.
///
/// `callstack_depth` specifies the maximum number of stack frames client should collect.
pub fn message(_message: &str, _callstack_depth: u16) {
}

/// Output a message with an associated color.
///
/// `callstack_depth` specifies the maximum number of stack frames client should collect.
///
/// The colour shall be provided as RGBA, where the least significant 8 bits represent the alpha
/// component and most significant 8 bits represent the red component.
pub fn color_message(_message: &str, _rgba: u32, _callstack_depth: u16) {
}

pub fn set_thread_name(_name: &str) {
}


/// Create an instance of plot that can plot arbitrary `f64` values.
///
/// # Examples
///
/// ```
/// # use tracy_client::*;
/// static TEMPERATURE: Plot = create_plot!("temperature");
/// TEMPERATURE.point(37.0);
/// ```
#[macro_export]
macro_rules! create_plot {
    ($name: literal) => {
        unsafe { $crate::Plot }
    };
}

/// A plot for plotting arbitary `f64` values.
///
/// Create with the [`create_plot`](create_plot) macro.
pub struct Plot;

impl Plot {
    /// Add a point with `y`-axis value of `value` to the plot.
    pub fn point(&self, _value: f64) {
    }
}
