use indicatif::{ProgressBar, ProgressStyle};
use std::{
    env::args,
    fs,
    path::Path,
    process::{exit, ExitCode},
    time::Duration,
};
use walkdir::WalkDir;

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    let src = args.get(1).unwrap().to_string();
    let dest = args.get(2).unwrap().to_string();
    let source = Path::new(src.as_str());
    let destination = Path::new(dest.as_str());
    if source.eq(destination) {
        println!("The source must be different of the destination");
        exit(1);
    }
    copy_directory(source, destination);
    exit(0);
}

fn copy_directory(source: &Path, destination: &Path) {
    let entries: Vec<_> = WalkDir::new(source)
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    let pb = ProgressBar::new(entries.len() as u64)
        .with_message("cloning")
        .with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.white} [ {eta} {elapsed} ]-[ {percent}% ]-[ {binary_bytes} {total_bytes} ] [{bar:50.white}] {msg}")
                .expect("")
                .progress_chars("## "),
        );

    pb.enable_steady_tick(Duration::from_millis(75));
    for entry in entries {
        let path = entry.path();
        let relative_path = path.strip_prefix(source).expect("msg");
        let destination_path = destination.join(relative_path);
        let x: String = String::from(relative_path.to_str().unwrap());
        if x.eq("lost+found") {
            continue;
        } else if destination_path.as_path().exists() {
            continue;
        }
        pb.set_message(x);
        if path.is_dir() {
            fs::create_dir_all(&destination_path).expect("");
            let permissions = fs::metadata(path).expect("").permissions();
            fs::set_permissions(&destination_path, permissions).expect("");
        } else if path.is_file() {
            copy_file_with_permissions(path, &destination_path);
        }
        pb.inc(1);
    }
    pb.finish_with_message(format!(
        "{} cloned to {}",
        source.to_str().unwrap(),
        destination.to_str().unwrap()
    ));
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
