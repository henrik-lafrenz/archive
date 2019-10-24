extern crate zip;

use std::{env, fs};
use std::io::Read;

use zip::read;


struct InfoText {
	path: std::path::PathBuf,
	text: String,
}


const PADDING: usize = 20;


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


fn print_results(info_texts: &std::vec::Vec<InfoText>, search_str: &String) {
	for info_text in info_texts.iter() {
		println!("\n[{}]\n", info_text.path.to_str().unwrap());

		let mut substr = info_text.text.clone();
		let mut index: usize = 0;
		loop {
			substr = substr[index..].to_string();
			let found = substr.to_lowercase().find(&search_str.to_lowercase());
			if found.is_some() {
				let begin = found.unwrap();
				let end = begin + search_str.len();
				let padded_begin = begin - std::cmp::min(begin, PADDING);
				let padded_end = end + std::cmp::min(substr.len() - begin - search_str.len(), PADDING);

				print!("{} --- ", substr[padded_begin..padded_end].to_string());
				index = end;
			} else {
				println!();
				break;
			}
		}
	}
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


fn main() {
	assert!(env::args().skip(1).len() == 2);
	let archive_path = std::path::PathBuf::from(env::args().nth(1).unwrap());
	let search_str = env::args().nth(2).unwrap();
	println!("\nSearching info texts in path {:?} for {:?}...",
		archive_path, search_str);

	let info_texts = collect_info_texts(&archive_path);
	print_results(&info_texts, &search_str);
	println!("...done.")
}
