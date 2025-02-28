#![no_std]

extern crate proc_macro;
pub use proc_macro::rename_future;

pub use elain::Align;

pub struct PhantomUnsend(core::marker::PhantomData<*const ()>);
unsafe impl core::marker::Sync for PhantomUnsend {}
