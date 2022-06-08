
pub trait IdInformation {
    fn g_vendor_id(&self) -> String;
    fn g_product_id(&self) -> String;
    fn g_name(&self) -> String;
}

#[derive(PartialEq)]
pub struct UsbLinkInfo {
    vendor_id: u16,
    product_id: u16,
}

impl UsbLinkInfo {
    pub fn new(vendor_id: u16, product_id: u16) -> UsbLinkInfo {
        UsbLinkInfo {
            vendor_id,
            product_id,
        }
    }
}

impl IdInformation for UsbLinkInfo {
    fn g_vendor_id(&self) -> String {
        self.vendor_id.to_string()
    }

    fn g_product_id(&self) -> String {
        self.product_id.to_string()
    }

    fn g_name(&self) -> String {
        String::new()
    }
}

pub fn scan_for_devices() -> Vec<UsbLinkInfo>{
    let mut retval = vec![];
    for devs in rusb::devices().unwrap().iter() {
        let dev_desc = devs.device_descriptor().unwrap();
        let link_info = UsbLinkInfo::new(dev_desc.vendor_id(), dev_desc.product_id());
        retval.push(link_info);
    }
    retval
}

