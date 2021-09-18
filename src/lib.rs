#[cfg(all(target_family = "unix"))]
pub mod convertor;

#[cfg(not(all(target_family = "unix")))]
pub mod convertor_win_mock;

#[cfg(not(all(target_family = "unix")))]
pub use convertor_win_mock as convertor;

#[cfg(all(target_family = "unix"))]
mod lame_ffi;
