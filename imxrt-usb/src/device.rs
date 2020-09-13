//! An [`usb-device`](https://crates.io/crates/usb-device) implementation
//! targeting processors in NXP's IMXRT family.

pub use imxrt_ral as ral;
pub use usb_device::{Result as UsbResult, UsbDirection, UsbError, bus::UsbBus, endpoint::EndpointAddress};

pub struct DeviceController {
    reg: ral::usb::Instance,
}

impl UsbBus for DeviceController {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8
    ) -> UsbResult<EndpointAddress> {
        unimplemented!()
    }

    fn enable(&mut self) {
        unimplemented!()
    }

    fn reset(&self) {
        unimplemented!()
    }

    fn set_device_address(&self, addr: u8) {
        unimplemented!()
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        unimplemented!()
    }

    fn read(&self,  ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        unimplemented!()
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        unimplmented!()
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        unimplemented!()
    }

    fn suspend(&self) {
        unimplemented!()
    }

    fn resume(&self) {
        unimplemented!()
    }

    fn poll(&self) -> PollResult {
        unimplmented!()
    }
}
