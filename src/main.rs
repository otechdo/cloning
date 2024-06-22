use ignore::{Walk, WalkBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::MAIN_SEPARATOR_STR;
use std::thread::sleep;
use std::{
    fs,
    path::Path,
    time::Duration,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Cloning src directory to dest", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        required = true,
        help = "The progress bar template",
    )]
    template: String,

    #[arg(
        short,
        long,
        required = true,
        help = "The progress bar progress chars")]
    progress: String,

    #[arg(short,
        long,
        required = true, help = "The source path")]
    src: String,

    #[arg(short, long, required = true, help = "The destination path")]
    dest: String,

    #[arg(short, long, help = "Enable watch mode", default_value_t = false)]
    watch: bool,

    #[arg(
        short,
        long,
        required = true,
        help = "The sleep time in ms (set it to 0 for better performances)"
    )]
    clock: u64,
}

fn walk(args: &Args) -> Walk
{
    WalkBuilder::new(&args.src)
        .threads(4)
        .standard_filters(true)
        .add_custom_ignore_filename("exclude.ji")
        .build()
}

fn copy(args: &Args)
{
    let x = walk(args);
    let src = args.src.to_string();
    let dest = args.dest.to_string();

    if Path::new(src.as_str()).exists().eq(&false) {
        println!("The source directory must be exist");
    }
    if src.eq(&dest) {
        println!("The source directory cannot be equals to destination");
    }
    let template = if args.template.is_empty() {
        "[{bar:70.white}] {binary_bytes_per_sec} | {total_bytes} {eta} {msg}".to_string()
    } else {
        args.template.to_string()
    };
    let progress = if args.progress.is_empty() {
        "== ".to_string()
    } else {
        args.progress.to_string()
    };

    copy_directory(
        src, dest, x, template, progress, args.clock)
}

fn main() {
    let args = Args::parse();
    if args.watch {
        loop {
            copy(&args);
        }
    } else {
        copy(&args);
    }
}

fn copy_directory(source: String, destination: String, e: Walk, template: String, progress_chars: String, i: u64) {
    let c: Vec<Result<ignore::DirEntry, ignore::Error>> = e.collect();
    let pb = ProgressBar::new(c.len() as u64)
        .with_message("cloning")
        .with_style(
            ProgressStyle::default_bar()
                .template(template.as_str()).expect("")
                .progress_chars(progress_chars.as_str()),
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
            pb.set_message(d.to_string());
            fs::create_dir_all(Path::new(d.as_str())).expect("");
            let permissions = fs::metadata(e.path()).expect("").permissions();
            fs::set_permissions(Path::new(d.as_str()), permissions).expect("");
        } else {
            pb.set_message(current.to_str().unwrap().to_string());
            copy_file_with_permissions(e.path(), Path::new(d.as_str()));
        }
        sleep(Duration::from_millis(i));
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
