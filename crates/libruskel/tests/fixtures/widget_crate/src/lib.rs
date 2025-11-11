pub struct Widget {
	pub id: u32,
	pub name: String,
}

impl Widget {
	/// Render the widget
	pub fn render(&self) -> String {
		todo!()
	}

	fn internal_helper(&mut self) {
		todo!()
	}
}

pub fn helper(widget: &Widget) -> Widget {
	todo!()
}

pub enum Palette {
	Named { label: String },
	Unspecified,
}

/// Utility helpers
pub mod tools {
	/// Instrument a widget
	pub fn instrument() {
		todo!()
	}
}
