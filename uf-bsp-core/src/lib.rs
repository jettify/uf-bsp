#![no_std]
#![doc = include_str!("../README.md")]

pub mod capabilities;
pub mod usb_cdc;

#[cfg(feature = "self-test")]
pub mod self_test;
