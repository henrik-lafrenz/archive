extern crate ansi_escapes;
extern crate zip;

use std::collections::VecDeque;
use std::vec::Vec;
use std::{env, fs, path};
use std::io::Read;


use zip::read;

mod data_types;


fn info_text(zip_path: &path::PathBuf) -> Option<String> {
	let file = fs::File::open(zip_path).expect("couldn't open zip path");
	let res = read::ZipArchive::new(file);
	let mut found = None;

	match res {
		Ok(mut zip) => {
			for i in 0..zip.len() {
				let res = zip.by_index(i);
				match res {
					Ok(mut zipped_file) => {
						if zipped_file.name().ends_with("info.txt") {

							let mut info_text = String::new();
							zipped_file.read_to_string(&mut info_text).expect(
								"couldn't read zipped file");

							found = Some(info_text);
							break;
						}
					},
					Err(e) => println!("couldn't get zipped file in {:?}: {:?}",
						zip_path, e),
				}
			}
		},
		Err(e) => println!("couldn't instantiate zip object for {:?}: {:?}",
			zip_path, e),
	}

	found
}


fn collect_info_texts(archive_path: &path::PathBuf) -> VecDeque<data_types::InfoText> {
	let mut info_texts :VecDeque<data_types::InfoText> = VecDeque::new();

	for e in fs::read_dir(archive_path).expect("couldn't read archive path") {
		let entry = e.expect("couldn't get entry");
		let item_path = entry.path();
		let ext = item_path.extension();
		if ext.is_some() && ext.unwrap() == "zip" {
			let it = info_text(&item_path);
			if it.is_some() {
				info_texts.push_back(data_types::InfoText{
						path: item_path,
						text: it.unwrap()
					});
			}
		}
	}

	info_texts
}


fn collect_search_results(
	info_texts: &mut VecDeque<data_types::InfoText>,
	search_str: &String) -> Option<Vec<data_types::SearchResult>> {

	let mut search_results :Vec<data_types::SearchResult> = Vec::new();

	loop {
		let result = info_texts.pop_front();
		match result {
			Some(info_text) => {
				let found = info_text.text.to_lowercase().find(
					&search_str.to_lowercase());

				if found.is_some() {
					search_results.push(
						data_types::SearchResult{
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


fn should_continue() -> bool {
	println!("? c(continue), q(quit)");
	let mut input = String::new();
	let boolean_res: bool;
	loop {
		std::io::stdin().read_line(&mut input).unwrap();
		if input.trim() == "c" {
			boolean_res = true;
			break;
		} else if input.trim() == "q" {
			boolean_res = false;
			break;
		} else {
			input.clear();
		}
	}

	boolean_res
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
			for (index, search_result) in search_results.iter().enumerate() {
				println!("{}\n", search_result);
				if index + 1 < search_results.len() && should_continue() {
					print!("{}", ansi_escapes::EraseLines(3));
					continue;
				} else {
					break;
				}


			}
		},
		None => println!("no search results"),
	}

	println!("...done.")
}
