use std::path::Path;
use std::env::current_dir;
use std::fs;
use clap::{Arg, App};

fn parse_args() -> Option<String>{
    let matches = App::new("wsl_pathable")
                          .version("0.1.0")
                          .author("persy")
                          .about("change Windows path to linux(wsl) path")
                          .arg(Arg::with_name("file_path")
                               .help("input a file path to change contents")
                               .takes_value(true))
                          .get_matches();
    let file_path: &str = matches.value_of("file_path").unwrap_or("compile_commands.json");
    Some(file_path.to_string())
}

fn run(file_path: &Path){
    let mut contents = fs::read_to_string(&file_path).expect(&format!("can read the file: {:?}", file_path));
    const WIN_ROOT: [&str; 6] = ["C:/", "D:/", "E:/", "F:/", "G:/", "H:/"];
    for r in WIN_ROOT{
        contents = contents.replace(r, &format!("/mnt/{}/", r.get(0..1).unwrap().to_lowercase()));
    }
    fs::write(file_path, &contents).expect(&format!("can not write to the file: {:?}", file_path));
}

fn main() {
    if let Some(file_path) = parse_args(){
        let path = Path::new(&file_path);
        if path.is_relative(){
            run(current_dir().unwrap().join(&path).as_path());
        } else{
            run(path);
        }
    }
}
