#[macro_use]
extern crate clap;
use clap::App;
use std::env;
use std::fs;
use std::fs::DirEntry;

fn main() {
    let yaml = load_yaml!("config.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let text = matches.value_of("text").unwrap_or("");
    let replace = matches.value_of("with").unwrap_or("");
    let prepend = matches.value_of("prepend").unwrap_or("");
    let cwd = env::current_dir().unwrap();
    let paths = fs::read_dir(cwd).unwrap();
    for path in paths {
        let file_name: String = get_file_name(path.unwrap());
        let replaced_name : String = str::replace(&file_name, text, replace);
        let updated_name = format!("{}{}", prepend, replaced_name);
        fs::rename(&file_name, &updated_name);
        println!("Old name: {}, New name: {}", file_name, updated_name);
    }
}

fn get_file_name(path: DirEntry) -> String {
        let file_name : String = path
            .file_name()
            .into_string()
            .expect("error: the first argument is not a file \
                    system path representable in UTF-8.");
        file_name
}