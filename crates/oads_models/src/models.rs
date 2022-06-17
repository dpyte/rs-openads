use crate::elephant::model::Elephant;

/// Centralized location containing models used in the project
pub struct ModelBank {
	elephant: Elephant
}

impl ModelBank {
	pub fn new() -> Self {
		let elephant = Elephant::new();
		Self { elephant }
	}
}
