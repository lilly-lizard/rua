use crate::{tar_check, wrapped};
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn action_jailbuild(offline: bool, dir: PathBuf, dirs: &ProjectDirs) {
	let dir_str = dir
		.to_str()
		.unwrap_or_else(|| panic!("{}:{} Cannot parse CLI target directory", file!(), line!()));
	wrapped::build_directory(dir_str, &dirs, offline);
	for file in dir.read_dir().expect("'target' directory not found") {
		tar_check::tar_check(
			&file
				.expect("Failed to open file for tar_check analysis")
				.path(),
		);
	}
	eprintln!("Package built and checked in: {}", dir_str);
	eprintln!("If you want to install the built artifacts, do it manually.");
}