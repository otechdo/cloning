use clap::{Arg, ArgMatches, Command};
use ignore::{Walk, WalkBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, set_permissions};
use std::io::{Error, ErrorKind};
use std::path::MAIN_SEPARATOR_STR;
use std::thread::sleep;
use std::{fs, path::Path, time::Duration};

fn cloning() -> ArgMatches {
    Command::new("cloning")
        .about("Cloning source directory to destination")
        .arg(
            Arg::new("template")
                .long("template")
                .require_equals(true)
                .required(true)
                .help("the progress console output"),
        )
        .arg(
            Arg::new("progress")
                .long("progress")
                .require_equals(true)
                .required(true)
                .help("the progress bar progress chars"),
        )
        .arg(
            Arg::new("source")
                .long("source")
                .require_equals(true)
                .required(true)
                .help("the directory to clone"),
        )
        .arg(
            Arg::new("destination")
                .long("destination")
                .require_equals(true)
                .required(true)
                .help("the directory destination path"),
        )
        .get_matches()
}

fn walk(source: &str) -> Walk {
    WalkBuilder::new(source)
        .threads(4)
        .standard_filters(true)
        .add_custom_ignore_filename("exclude.ji")
        .build()
}

fn copy(source: &str, destination: &str, template: &str, progress: &str) -> Result<(), Error> {
    if Path::new(source).exists().eq(&false) {
        return Err(Error::new(
            ErrorKind::NotFound,
            "The source directory must be exist",
        ));
    }

    if source.eq(destination) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "The source directory must be different of the dest",
        ));
    }
    copy_directory(source, destination, walk(source), template, progress);
    Ok(())
}

fn main() -> Result<(), Error> {
    let app = cloning();
    if let Some(source) = app.get_one::<String>("source") {
        if Path::new(source).exists().eq(&false) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "The source directory must be exist",
            ));
        }
        if let Some(destination) = app.get_one::<String>("destination") {
            if let Some(template) = app.get_one::<String>("template") {
                if let Some(progress) = app.get_one::<String>("progress") {
                    return copy(
                        source.as_str(),
                        destination.as_str(),
                        template.as_str(),
                        progress.as_str(),
                    );
                }
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "The progress chars must be defined",
                ));
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "The template must be defined",
            ));
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "The destination directory must be defined",
        ));
    }
    Err(Error::new(
        ErrorKind::InvalidInput,
        "The source directory must be defined",
    ))
}

fn cp(s: &Path, d: &Path) {
    assert!(fs::copy(s, d).is_ok());
    if let Ok(data) = s.metadata() {
        let perms = data.permissions();
        assert!(set_permissions(d, perms).is_ok());
    }
}
fn copy_directory(source: &str, destination: &str, e: Walk, template: &str, progress_chars: &str) {
    let c: Vec<Result<ignore::DirEntry, ignore::Error>> = e.collect();
    let pb = ProgressBar::new(c.len() as u64)
        .with_message("Cloning")
        .with_style(
            ProgressStyle::default_bar()
                .template(template)
                .expect("Template not valid")
                .progress_chars(progress_chars),
        );

    pb.enable_steady_tick(Duration::from_millis(75));

    for x in c {
        let e = x.unwrap();
        let path = e.path();
        let current = e.path().file_name().unwrap();
        let dest = path.strip_prefix(source).unwrap();
        let mut d = String::from(destination);
        d.push_str(MAIN_SEPARATOR_STR);
        d.push_str(dest.to_str().unwrap());
        if path.is_dir() {
            pb.set_message(d.to_string());
            assert!(create_dir_all(Path::new(d.as_str())).is_ok());
            if let Ok(data) = e.metadata() {
                let perms = data.permissions();
                assert!(set_permissions(path, perms).is_ok());
            }
        } else {
            if let Some(current) = current.to_str() {
                pb.set_message(current.to_string());
            }
            copy_file_with_permissions(e.path(), Path::new(d.as_str()));
        }
        sleep(Duration::from_millis(250));
        pb.inc(1);
    }
    pb.finish_with_message("Cloned");
}

fn copy_file_with_permissions(source: &Path, destination: &Path) {
    if Path::new(destination).exists().eq(&false) {
        if let Some(parent) = destination.parent() {
            assert!(create_dir_all(parent).is_ok());
        }
    }
    cp(source, destination);
}
