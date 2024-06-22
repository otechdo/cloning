use ignore::{Walk, WalkBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::MAIN_SEPARATOR_STR;
use std::thread::sleep;
use std::{
    env::args,
    fs,
    path::Path,
    process::{exit, ExitCode},
    time::Duration,
};

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    let src = args.get(1).unwrap().to_string();
    let dest = args.get(2).unwrap().to_string();
    let source = Path::new(src.as_str());
    let destination = Path::new(dest.as_str());
    if source.exists().ne(&true) {
        println!("Source must be exists");
        exit(1);
    }
    if source.eq(destination) {
        println!("The source must be different of the destination");
        exit(1);
    }
    if source.is_file() {
        println!("Source must be a directory");
        exit(1);
    }
    let x = WalkBuilder::new(source)
        .threads(4)
        .standard_filters(true)
        .add_custom_ignore_filename("exclude.ji")
        .build();
    copy_directory(src, dest, x);
    exit(0);
}

fn copy_directory(source: String, destination: String, e: Walk) {
    let c: Vec<Result<ignore::DirEntry, ignore::Error>> = e.collect();
    let pb = ProgressBar::new(c.len() as u64)
        .with_message("cloning")
        .with_style(
            ProgressStyle::default_bar()
                .template("[ {percent}% ]-[ {binary_bytes} {total_bytes} ]-[{bar:50.white}] {msg}")
                .expect("")
                .progress_chars("=-"),
        );

    pb.enable_steady_tick(Duration::from_millis(75));

    for x in c {
        let e = x.unwrap();
        let path = e.path();
        let current = e.path().file_name().unwrap();
        let dest = path.strip_prefix(source.as_str()).unwrap();
        let mut d = String::from(destination.as_str());
        d.push_str(MAIN_SEPARATOR_STR);
        d.push_str(dest.to_str().unwrap());
        if path.is_dir() {
            pb.set_message(format!("Creating {} directory", d.to_string()));
            fs::create_dir_all(Path::new(d.as_str())).expect("");
            let permissions = fs::metadata(e.path()).expect("").permissions();
            fs::set_permissions(Path::new(d.as_str()), permissions).expect("");
        } else {
            pb.set_message(format!("Copying {} file", current.to_str().unwrap()));
            copy_file_with_permissions(e.path(), Path::new(d.as_str()));
        }
        sleep(Duration::from_millis(500));
        pb.inc(1);
    }
    pb.finish_with_message(format!(
        "{} cloned to {}",
        Path::new(source.as_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        Path::new(destination.as_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
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
