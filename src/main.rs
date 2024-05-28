use indicatif::{ProgressBar, ProgressStyle};
use std::{env::args, fs, path::Path};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = args().collect();
    let source_directory = Path::new(args.get(1).unwrap().as_str()); // Replace with your source directory path
    let destination_directory = Path::new(args.get(2).unwrap().as_str()); // Replace with your source directory path
    copy_directory(source_directory, destination_directory);
}

fn copy_directory(source: &Path, destination: &Path) {
    let entries: Vec<_> = WalkDir::new(source)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let pb = ProgressBar::new(entries.len() as u64).with_message("cloning");
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[ {eta} / {elapsed} ] [{bar:50.white}] {msg}")
            .expect("")
            .progress_chars("=-"),
    );

    for entry in entries {
        let path = entry.path();
        let relative_path = path.strip_prefix(source).expect("msg");
        let destination_path = destination.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&destination_path).expect("");
            let permissions = fs::metadata(path).expect("").permissions();
            fs::set_permissions(&destination_path, permissions).expect("");
        } else if path.is_file() {
            copy_file_with_permissions(path, &destination_path);
        }
        pb.inc(1);
    }
    pb.finish_with_message("cloned");
}

fn copy_file_with_permissions(source: &Path, destination: &Path) {
    if !Path::new(destination).exists() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).expect("");
        }
        assert!(fs::copy(source, destination).is_ok());
        let permissions = fs::metadata(source).expect("").permissions();
        fs::set_permissions(destination, permissions).expect("msg");
    }
}
