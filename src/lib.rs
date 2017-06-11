extern crate opencl;
extern crate libc;

pub use context::Context;
pub use array::Array;
pub use tensor::{Event, Tensor, TensorMode};
pub use ops::*;
pub use range_arg::RangeArg;

pub mod array;
pub mod context;
pub mod kernels;
pub mod num;
#[macro_use] pub mod range_arg;
pub mod ops;
pub mod tensor;

mod helper;
