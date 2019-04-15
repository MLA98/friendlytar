use std::fs;
use std::process::Command;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    if !path_exists(&args.target) {
        panic!("The target file does not exist!")
    }

    let md = fs::metadata(&args.target).unwrap();
    if md.is_dir() {
        let _output = Command::new("tar")
            .arg("-cvf")
            .arg(format!("{}.tar", &args.target))
            .arg(&args.target)
            .output()
            .expect("failed to execute process");
    } else {
        let file_type = Command::new("file")
            .arg(&args.target)
            .output()
            .expect("failed to execute process");
        if String::from_utf8_lossy(&file_type.stdout).contains("POSIX tar archive") {
            let _output = Command::new("tar")
                .arg("-xvf")
                .arg(&args.target)
                .output()
                .expect("failed to execute process");
        } else {
            println!("This is neither a folder or a tar file!")
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    target: String,
    // #[structopt(parse(from_os_str))]
    // path: std::path::PathBuf,
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
