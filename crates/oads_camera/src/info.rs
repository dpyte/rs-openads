use crate::scan::{IdInformation, UsbLinkInfo};

#[derive(Clone)]
pub enum Protocol {
    USB,
    ETH
}

#[derive(Clone)]
pub enum Status {
    OFFLINE,
    ONLINE,
}

#[derive(Clone)]
pub enum ConnectionType {
    SSH,
    RTSP,
    HARD, // Wired
}

#[derive(Clone)]
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

    pub fn update_id(&mut self, id: String) {
        self.id = id
    }

    pub fn g_id(&self) -> String { self.id.clone() }

    pub fn g_connection_type(&self) -> String {
        let retval = match self.connection_type {
            ConnectionType::SSH => String::from("SSH"),
            ConnectionType::RTSP => String::from("RTSP"),
            ConnectionType::HARD => String::from("HARD"),
        };
        retval
    }
}

impl IdInformation for CameraInfo {
    fn g_vendor_id(&self) -> String { self.vendor_id.to_string() }
    fn g_product_id(&self) -> String { self.product_id.to_string() }
    fn g_name(&self) -> String { self.name.to_string() }

}
