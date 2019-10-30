extern crate ansi_term;
extern crate zip;

use std::{env, fs};
use std::fmt;
use std::io::Read;

use ansi_term::Colour::{Green, Yellow};

use zip::read;


#[derive(Clone)]
struct InfoText {
	path: std::path::PathBuf,
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
		let mut substr = self.info_text.text.clone();
		loop {
			let found = substr.to_lowercase().find(search_str_lower);
			match found {
				Some(index) => {
					let mut search_str_case = substr.split_off(index);
					let remain = search_str_case.split_off(self.search_str.len());
					write!(f, "{}{}",
						substr, Yellow.underline().paint(search_str_case))?;
					substr = remain;
				},
				None => {
					write!(f, "{}", substr)?;
					break;
				},
			}
		}

		Ok(())
	}
}


fn info_text(zip_path: &std::path::PathBuf) -> Option<String> {
	let file = std::fs::File::open(zip_path).expect("couldn't open zip path");
	let mut zip = read::ZipArchive::new(file).expect("couldn't instantiate zip");
	let mut found = None;

	for i in 0..zip.len() {
		let mut zipped_file = zip.by_index(i).expect("couldn't get zipped file");
		let find_i = zipped_file.name().find("info.txt");
		if find_i.is_some() {
			let mut buffer = String::new();
			let res = zipped_file.read_to_string(&mut buffer);
			match res {
				Ok(_v) => {
					found = Some(buffer);
					break;
				},
				Err(e) => println!("couldn't read info text: {:?}", e),
			}
		}
	}

	found
}


fn collect_info_texts(archive_path: &std::path::PathBuf) -> std::vec::Vec<InfoText> {
	let mut info_texts :std::vec::Vec<InfoText> = std::vec::Vec::new();
	for e in fs::read_dir(archive_path).expect("couldn't read archive path") {
		let entry = e.expect("couldn't get entry");
		let item_path = entry.path();
		let ext = item_path.extension();
		if ext.is_some() && ext.unwrap() == "zip" {
			let it = info_text(&item_path);
			if it.is_some() {
				info_texts.push(InfoText{path: item_path, text: it.unwrap()});
			}
		}
	}

	info_texts
}


fn collect_search_results(info_texts: std::vec::Vec<InfoText>, search_str: &String)
-> Option<std::vec::Vec<SearchResult>> {
	let mut search_results :std::vec::Vec<SearchResult> = std::vec::Vec::new();

	for info_text in info_texts.iter() {
		let found = info_text.text.to_lowercase().find(&search_str.to_lowercase());
		if found.is_some() {
			search_results.push(
				SearchResult{
					info_text: info_text.clone(),
					search_str: search_str.clone(),
				});
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
	let archive_path = std::path::PathBuf::from(env::args().nth(1).unwrap());
	let search_str = env::args().nth(2).unwrap();
	println!("\nSearching info texts in path {:?} for {:?}...",
		archive_path, search_str);

	let info_texts = collect_info_texts(&archive_path);

	let search_results = collect_search_results(info_texts, &search_str);
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
