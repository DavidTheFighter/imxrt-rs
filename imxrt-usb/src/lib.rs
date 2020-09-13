//! NXP iMXRT USB driver
//!
//! The module provides an abstraction over a EHCI USB Controller which may act either
//! as a host, device, or otg (pin selected host/device)

#![no_std]
use core::marker::PhantomData;
use imxrt_ral as ral;
use imxrt_iomuxc::{self as iomuxc, consts::{Unsigned, U1, U2}};

pub use device;

pub struct Unclocked<M> where M: Unsigned {
    pub (crate) _module: PhantomData<M>,
    pub (crate) reg: ral::usb::Instance;
}

impl Unclocked<M> {
    /// Enable the clock for a particular USB device
    pub fn clock(
        self,
        handle: &mut ccm::Handle,
    ) -> Builder<M> {
        //TODO enable matching USB PLL here
        Builder::new(self.dev)
    }
}

pub struct Builder<M> where M: Unsigned 
{

    //TODO pub fn unclock(self) -> Unclocked<M> {...}
    
    //TODO pub fn otg(self) -> OTG<M> {...}
    
    //TODO pub fn host(self) -> Host<M> {...}
    
    pub fn device(self) -> Device<M>
    {
        Device::new(self.reg);
    }
}
