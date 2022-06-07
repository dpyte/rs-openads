use std::fs;
use std::fs::File;
use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use uuid::Uuid;

use crate::info::{CameraInfo, ConnectionType};
use crate::scan::{IdInformation, scan_for_devices, UsbLinkInfo};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Fr,
    De,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Cameras {
    camera: Vec<Camera>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Camera {
    name: String,
    device_count: String,
    device_type: String,
    vendor_id: String,
    product_id: String,
    id: Option<String>,
}

fn read_xml_data(read_from: &String) -> Vec<CameraInfo> {
    let contents = fs::read_to_string(read_from).expect("file not found\n");
    let cam_list: Cameras = from_str(&contents).unwrap();

    let mut retval = Vec::new();
    for x in cam_list.camera {
        let mut id_is: String = String::new();
        match x.id {
            Some(x) => id_is = x,
            None => {
                id_is = Uuid::new_v4().to_string();
                /// !TODO: update to log
                println!("updating new uuid for {}: {}", x.name, id_is);
            },
        };

        let extracted_values = CameraInfo::new(
            &x.name,
            &String::new(),
            ConnectionType::HARD,
            0,
            &x.vendor_id,
            &x.product_id,
            id_is
        );
        retval.push(extracted_values);
    }
    retval
}

pub struct Read {
    info_file: String,
    xcinfo: Vec<CameraInfo>,
    validated_devices: Vec<CameraInfo>,
}

impl Read {
    pub fn new() -> Read {
        Read {
            info_file: String::from("/var/system/openads/config/camera/info/info.xml"),
            xcinfo: vec![],
            validated_devices: vec![]
        }
    }

    fn validate_devices(&mut self) -> Vec<CameraInfo> {
        let _validated_devs: Vec<UsbLinkInfo> = scan_for_devices();
        let _xdata = &&read_xml_data(&self.info_file);

        let mut v_device: Vec<CameraInfo> = Vec::new();
        for vd in _validated_devs {
            for xd in _xdata.iter() {
                if *xd == vd {
                    v_device.push(*xd)
                }
            }
        }
        v_device
    }
}
