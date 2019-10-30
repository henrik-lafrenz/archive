extern crate ansi_term;
extern crate zip;

use std::collections::VecDeque;
use std::vec::Vec;
use std::{env, fmt, fs, path};
use std::io::Read;

use ansi_term::Colour::{Green, Yellow};

use zip::read;


struct InfoText {
	path: path::PathBuf,
	text: String,
}


struct SearchResult {
	info_text: InfoText,
	search_str: String,
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


fn info_text(zip_path: &path::PathBuf) -> Option<String> {
	let file = fs::File::open(zip_path).expect("couldn't open zip path");
	let mut zip = read::ZipArchive::new(file).expect("couldn't instantiate zip");
	let mut found = None;

	for i in 0..zip.len() {
		let mut zipped_file = zip.by_index(i).expect("couldn't get zipped file");
		if zipped_file.name().ends_with("info.txt") {
			let mut info_text = String::new();
			let res = zipped_file.read_to_string(&mut info_text);
			match res {
				Ok(_v) => {
					found = Some(info_text);
					break;
				},
				Err(e) => println!("couldn't read info text: {:?}", e),
			}
		}
	}

	found
}


fn collect_info_texts(archive_path: &path::PathBuf) -> VecDeque<InfoText> {
	let mut info_texts :VecDeque<InfoText> = VecDeque::new();

	for e in fs::read_dir(archive_path).expect("couldn't read archive path") {
		let entry = e.expect("couldn't get entry");
		let item_path = entry.path();
		let ext = item_path.extension();
		if ext.is_some() && ext.unwrap() == "zip" {
			let it = info_text(&item_path);
			if it.is_some() {
				info_texts.push_back(InfoText{path: item_path, text: it.unwrap()});
			}
		}
	}

	info_texts
}


fn collect_search_results(info_texts: &mut VecDeque<InfoText>, search_str: &String)
-> Option<Vec<SearchResult>> {
	let mut search_results :Vec<SearchResult> = Vec::new();

	loop {
		let result = info_texts.pop_front();
		match result {
			Some(info_text) => {
				let found = info_text.text.to_lowercase().find(
					&search_str.to_lowercase());

				if found.is_some() {
					search_results.push(
						SearchResult{
							info_text: info_text,
							search_str: search_str.clone(),
						});
				}
			},
			None => break,
		}
	}

	if !search_results.is_empty() {
		Some(search_results)
	} else {
		None
	}
}


fn main() {
	assert!(env::args().skip(1).len() == 2);
	let archive_path = path::PathBuf::from(env::args().nth(1).unwrap());
	let search_str = env::args().nth(2).unwrap();

	println!("\nSearching info texts in path {:?} for {:?}...",
		archive_path, search_str);

	let mut info_texts = collect_info_texts(&archive_path);
	let search_results = collect_search_results(&mut info_texts, &search_str);
	match search_results {
		Some(search_results) => {
			println!();
			for search_result in search_results.iter() {
				println!("{}\n", search_result);
			}
		},
		None => println!("no search results"),
	}

	println!("...done.")
}
