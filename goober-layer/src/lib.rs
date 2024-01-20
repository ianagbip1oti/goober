mod add;
mod conv1d;
mod dense;
mod sparse;

pub use add::Add;
pub use conv1d::Conv1D;
pub use dense::DenseConnected;
pub use sparse::SparseConnected;

fn boxed_and_zeroed<T>() -> Box<T> {
    unsafe {
        let layout = std::alloc::Layout::new::<T>();
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Box::from_raw(ptr.cast())
    }
}
