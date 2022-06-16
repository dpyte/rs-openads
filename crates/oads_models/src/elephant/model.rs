use std::fs::File;
use tch::nn::VarStore;
use log::{debug, warn};
use std::collections::HashMap;
use std::io::{BufReader, Lines};

// Path to pretrained model
static PRETRAINED_MODEL: &str = "/var/system/openads/config/models/elephant/scad_elephantnn_model.pt";
// Lets the system know whether to execute the model on CPU or GPU
static MODEL_EXECUTION_TYPE: &str = "/var/system/openads/config/models/elephant/elephant.config";

fn process_config_file(lines: Lines<BufReader<File>>, h_map: &mut HashMap<String, String>) {
	let mut content: Vec<String> = vec![];
	for line in lines {
		if let Ok(line) = line {
			let first_split: Vec<&str> = line.split("=").collect();
			match first_split[0].to_string().as_str() {
				"executor" => {
					let key = String::from(first_split[0].to_string());
					let value = String::from(first_split[1].to_string());
					debug!("Inserting {:?} -> {:?}", key, value);
					h_map.insert(key, value);
				},
				_ => { }
			}
		}
	}
}

fn get_model_params() -> HashMap<String, String> {
	let mut retval = HashMap::new();
	let data: Lines<BufReader<File>>;
	match crate::read_lines(MODEL_EXECUTION_TYPE) {
		Ok(buffer) => {
			process_config_file(buffer, &mut retval);
		},
		Err(_) => {
			warn!("Unable to open {}. Use default params ...", MODEL_EXECUTION_TYPE);
			retval.insert(String::from("executor"), String::from("CPU"));
		}
	};
	retval
}

/// Core component of this file - contains pretrained model and corresponding data
pub struct Elephant {
	varstore: VarStore,
}

impl Elephant {
	pub fn new() -> Self {
		let params = get_model_params();

		let device_type = match params.get("executor").unwrap().to_string().as_str() {
			// Current model is restricted to rely on CPU to execute current model
			_ => tch::Device::Cpu
		};
		warn!("Setting device type for elephant to {:?}", device_type);
		let varstore = VarStore::new(device_type);

		// !FIXME
		// Current ptm is causing Runtime error possibly due to the ptm being trained on CUDA instead of using CPU.
		// openads-models will have a cpu-based model available in its next pr.
		let model = tch::CModule::load(PRETRAINED_MODEL).expect("FAILURE loading pretrained model for Elephant");
		debug!("Successfully loaded model for elephant with @params {:?}", model);

		Self { varstore }
	}
}
