use rusb::{Device, DeviceDescriptor, DeviceHandle, GlobalContext};
use std::time::Duration;

pub struct MouseDevice {
    device: Device<GlobalContext>,
    device_desc: DeviceDescriptor,
    handle: DeviceHandle<GlobalContext>,
}

impl MouseDevice {
    pub fn new() -> Result<Self, rusb::Error> {
        let vid = 0x258a;
        let pid = 0x1007;

        let devices = rusb::DeviceList::new()?;

        let device = devices.iter().find(|d| match d.device_descriptor() {
            Ok(d) => d.vendor_id() == vid && d.product_id() == pid,
            Err(_) => false,
        });

        let device = if let Some(device) = device {
            device
        } else {
            return Err(rusb::Error::NotFound);
        };

        let handle = device.open()?;

        Ok(Self {
            device_desc: device.device_descriptor()?,
            device,
            handle,
        })
    }
    pub fn kernel_detach(&mut self) -> Result<(), rusb::Error> {
        let nc = self.device_desc.num_configurations();

        for n in 0..nc {
            let cd = self.device.config_descriptor(n)?;
            for i in cd.interfaces() {
                if self.handle.kernel_driver_active(i.number()).is_ok() {
                    self.handle.detach_kernel_driver(i.number()).ok();
                }
            }
        }

        Ok(())
    }
    pub fn kernel_attach(&mut self) -> Result<(), rusb::Error> {
        self.handle.attach_kernel_driver(0)?;
        Ok(())
    }
    pub fn read(&mut self) -> Result<crate::protocol::ConfigData, rusb::Error> {
        let mut out: [u8; 154] = [0; 154];

        self.handle
            .read_control(0xa1, 0x01, 0x304, 1, &mut out, Duration::from_secs(1))?;

        let data: crate::protocol::ConfigData = unsafe { std::mem::transmute(out) };

        Ok(data)
    }
    pub fn send(&mut self, config_data: &crate::protocol::ConfigData) -> Result<(), rusb::Error> {
        let data: &[u8; 154] = unsafe { std::mem::transmute(config_data) };

        self.handle
            .write_control(0x21, 0x09, 0x0304, 1, data, Duration::from_secs(1))?;

        Ok(())
    }
}
