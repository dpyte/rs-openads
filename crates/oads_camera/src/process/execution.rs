use crate::process::execution::RuntimeMode::{Execution, Learning};
use crate::read_lines;

/// Path to execution mode file. This file lets the system know whether it is executing in learning mode or execution mode.
static PATH_TO_EXECUTION_MODE: &str = "/var/system/openads/execution/exec";

/// Learning Mode  -> Familiarizes itself with background
/// Execution Mode ->

/// Runtime Mode
#[derive(Debug, Clone)]
pub enum RuntimeMode {
	Learning,
	Execution,
}

pub fn execution_mode() -> RuntimeMode {
	let mut retval = RuntimeMode::Execution;
	let lines = read_lines(PATH_TO_EXECUTION_MODE);
	for line in lines.expect("Unable to open file") {
		if let Ok(line) = line {
			retval = match line.as_str() {
				"LEARNING"  => Learning,
				_ => Execution,
			};
			break;
		}
	}
	retval
}
