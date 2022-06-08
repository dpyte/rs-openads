use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Cursor, Write};
use std::ptr::write;
use uuid::Uuid;
use quick_xml::{Reader, Writer};
use serde::Deserialize;
use quick_xml::de::from_str;
use quick_xml::events::Event;

use crate::info::{CameraInfo, ConnectionType};
use crate::info::ConnectionType::{HARD, RTSP, SSH};
use crate::scan::{IdInformation, scan_for_devices, UsbLinkInfo};

const READ_FROM: &str = "/var/system/openads/config/camera/info/info.xml";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Fr,
    De,
}

#[derive(Debug, Deserialize, Clone)]
struct Cameras {
    camera: Vec<Camera>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Camera {
    pub name: String,
    pub device_count: String,
    pub device_type: String,
    pub vendor_id: String,
    pub product_id: String,
    pub id: Option<String>,
}

impl Camera {
    pub fn update_id(&mut self, id: String) {
        self.id = Option::from(id);
    }
}

fn read_xml_data(read_from: &str) -> Vec<Camera> {
    let contents = fs::read_to_string(read_from).expect("file not found\n");
    let read = from_str::<Cameras>(&contents);
    let data = match read {
        Ok(data) => data,
        Err(e)   => panic!("\n\n{:?}\n\n", e),
    };
    data.camera
}

pub struct Read {
    xml_read_data: Vec<Camera>,
    validated_devices: Vec<CameraInfo>
}

impl Read {
    pub fn new() -> Read {
        Read {
            xml_read_data: read_xml_data(READ_FROM),
            validated_devices: vec![]
        }
    }

    pub fn validate_and_match(&mut self) {
        // Scan for available devices and matches it against the xml data
        // Matched value can later be accessed to perform write to the XML data
        // Return a confirmed DOM structure <XML> back to the calling function
        let scanned_devices: Vec<UsbLinkInfo> = scan_for_devices();
        let xml_data: &Vec<Camera> = &self.xml_read_data;

        let mut x_scan_counter: usize = 0;
        let mut x_scan_idxs = vec![];
        for xd in xml_data {
            let vendor_id = xd.vendor_id.parse::<u16>().unwrap();
            let product_id = xd.product_id.parse::<u16>().unwrap();

            let tmp_structure = UsbLinkInfo::new(vendor_id, product_id);
            let search_result = scanned_devices.iter().position(|r| r == &tmp_structure);
            let contained_index = match search_result {
                Some(index) => index,
                _ => 99
            };

            if contained_index != 99 {
                x_scan_idxs.push(x_scan_counter);
            }
            x_scan_counter += 1;
        }

        for i in x_scan_idxs {
            let mut x_data = &mut self.xml_read_data[i];
            let id = match &x_data.id {
                Some(id) => id.to_string(),
                None => Uuid::new_v4().to_string()
            };

            println!("detected device: \n*\t{}\n\tvendor id: {}\n\tproduct id: {}\n\tid: {}\n",
                     x_data.name, x_data.vendor_id, x_data.product_id, id);

            x_data.update_id(id.clone());
            let allocate_new = CameraInfo::new(&x_data.name, &String::new(), ConnectionType::HARD,
                                               0, &x_data.vendor_id, &x_data.product_id, id);
            self.validated_devices.push(allocate_new);
        }
    }

    pub fn save_updated_ids(&mut self) {
        // Update xml scanned data with validated
        // we know that validated <= read xml data
        for x in &self.xml_read_data{
            /*
            <cameras>
                <camera name="" device_count="001" device_type="" vendor_id="" product_id="" id=""/>
            </cameras>
            */
            let id = match &x.id {
                Some(x) => x,
                None => ""
            };
            let write_back = format!(
                "<cameras>\n\t<camera name={:?} device_count=001 device_type={:?} vendor_id={:?} product_id={:?} id={:?}/>\n</cameras>\n",
                x.name,
                x.device_type,
                x.vendor_id,
                x.product_id,
                id
            );
            let write_to = OpenOptions::new()
                .append(true)
                .create(true)
                .open("test.xml")
                .expect("Unable to open file");
            let mut write_to = BufWriter::new(write_to);
            write_to.write_all(write_back.as_bytes()).expect("Unable to write data");
        }
    }

    pub fn device_count(&self) -> usize {
        self.validated_devices.len()
    }

    pub fn validated_cameras(&self) -> &Vec<CameraInfo> {
        &self.validated_devices
    }
}
