use std::collections::HashMap;
use tch::nn::VarStore;


// Lets the system know whether to execute the model on CPU or GPU
static MODEL_EXECUTION_TYPE: &str = "/var/system/openads/config/models/elephant/elephant.config";

fn get_model_params() -> HashMap<String, String> {

	let mut retval = HashMap::new();

	let mut use_config_data = true;
	let data = match crate::read_lines(MODEL_EXECUTION_TYPE) {
		Ok(buffer) => buffer,
		Err(_) => { use_config_data = false; }
	};

	if !use_config_data {
		warn!("Unable to open {}. Use default params ...", MODEL_EXECUTION_TYPE);
		retval.insert(String::from("executor"), String::from("cpu"));
	} else {

	}

	retval
}


pub struct Elephant {
	varstore: VarStore,
}

impl Elephant {
	pub fn new() -> Self {
		let varstore = tch::nn::VarStore::new(tch::Device::Cpu);
		Self { varstore }
	}
}
