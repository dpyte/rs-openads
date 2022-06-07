use crate::scan::{IdInformation, UsbLinkInfo};

pub enum Protocol {
    USB,
    ETH
}

pub enum Status {
    OFFLINE,
    ONLINE,
}

pub enum ConnectionType {
    SSH,
    RTSP,
    HARD, // Wired
}

pub struct CameraInfo {
    name: String,
    address: String,
    connection_type: ConnectionType, // ssh, rtsp, ...
    port: u16,
    vendor_id: String,
    product_id: String,
    id: String,
    protocol: Protocol,
    status: Status,
}

impl CameraInfo {
    pub fn new(name: &String, address: &String, connection_type: ConnectionType, port: u16, vendor_id: &String, product_id: &String, id: String) -> CameraInfo {
        CameraInfo {
            name: name.to_string(),
            address: address.to_string(),
            connection_type,
            port,
            vendor_id: vendor_id.to_string(),
            product_id: product_id.to_string(),
            id,
            protocol: Protocol::USB,
            status: Status::OFFLINE,
        }
    }
}

impl IdInformation for CameraInfo {
    fn g_vendor_id(&self) -> String {
       self.vendor_id.to_string()
    }

    fn g_product_id(&self) -> String {
        self.product_id.to_string()
    }

    fn g_name(&self) -> String {
        self.name.to_string()
    }
}

impl<'a, 'b> PartialEq<UsbLinkInfo> for CameraInfo {
    fn eq(&self, other: &UsbLinkInfo) -> bool {
        self.product_id == other.g_product_id()
            && self.vendor_id == other.g_vendor_id()
    }
}
