extern crate ansi_term;

use std::{fmt, path};

use ansi_term::Colour::{Green, Yellow};


pub struct InfoText {
	pub path: path::PathBuf,
	pub text: String,
}


pub struct SearchResult {
	pub info_text: InfoText,
	pub search_str: String,
}


impl fmt::Display for SearchResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n", Green.paint(self.info_text.path.to_str().unwrap()))?;

		let search_str_lower = &self.search_str.to_lowercase();
		let mut before = self.info_text.text.clone();

		loop {
			let found = before.to_lowercase().find(search_str_lower);
			match found {
				Some(index) => {
					let mut search_str_case = before.split_off(index);
					let after = search_str_case.split_off(self.search_str.len());
					write!(f, "{}{}",
						before, Yellow.underline().paint(search_str_case))?;
					before = after;
				},
				None => {
					write!(f, "{}", before)?;
					break;
				},
			}
		}

		Ok(())
	}
}