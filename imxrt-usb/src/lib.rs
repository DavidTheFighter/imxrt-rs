//! NXP iMXRT USB driver
//!
//! The module provides an abstraction over a EHCI USB Controller which may act either
//! a host or device

#![no_std]
use core::marker::PhantomData;
pub use imxrt_ral as ral;

pub trait ControllerMode {}

pub enum Disabled {}
pub enum Device {}
pub enum Host {}

impl ControllerMode for Disabled {}
impl ControllerMode for Device {}
impl ControllerMode for Host {}

pub struct DeviceController<M: ControllerMode> {
    _mode: PhantomData<M>,
    reg: ral::usb::Instance,
}

impl Controller {
    fn device(reg: ral::usb::Instance
}

pub use device;
