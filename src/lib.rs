#![no_std]
#![allow(clippy::type_complexity)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod cache;
pub mod consumer;
pub mod local;
pub mod observer;
pub mod producer;
pub mod raw;
pub mod ring_buffer;
pub mod shared;
pub mod storage;
pub mod stored;
mod transfer;
mod utils;

#[cfg(test)]
mod tests;

pub use consumer::Consumer;
pub use local::LocalRb;
pub use observer::Observer;
pub use producer::Producer;
pub use ring_buffer::{RingBuffer, Split};
pub use shared::SharedRb;
pub use transfer::transfer;

pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use crate::stored::HeapRb;
    pub use crate::{
        consumer::{ByteConsumer, Consumer},
        observer::Observer,
        producer::{ByteProducer, Producer},
        ring_buffer::{RingBuffer, Split},
        stored::StaticRb,
    };
}
