use std::*;
use std::result::Result::*;
use std::fs;
use std::path::Path;
use std::io::prelude::*;

fn log_error(msg: String) { eprintln!("error: {}", msg) }

fn get_file_content(fpath: &Path) -> std::result::Result<String, String> {
    if !fpath.is_file() { return Err(format!("\"{}\" is not file", fpath.display())); }
    if !fpath.exists() { return Err(format!("\"{}\" is not exist", fpath.display())); }
    let mut file = fs::File::open(fpath).ok().unwrap();
    let mut content = String::new();
    let res = file.read_to_string(&mut content);
    if res.is_err() {
        return std::result::Result::Err(format!("error: reading file {} finished with error {}", fpath.display(), res.unwrap()));
    }
    return Ok(content);
}

fn convert_toml2json(content: String) -> String {
    let config: toml::Value = toml::from_str(content.as_str()).unwrap();
    return serde_json::to_string_pretty(&config).ok().unwrap();
}

fn convert_json2toml(content: String) -> String {
    let config: toml::Value = serde_json::from_str(content.as_str()).unwrap();
    return toml::ser::to_string_pretty(&config).ok().unwrap();
}

fn parse_file(file: &str) -> std::result::Result<(), String> {
    let fpath = Path::new(file);
    let content = get_file_content(fpath).map_err(|x| { log_error(x); }).ok();
    match content {
        Some(_) => {
            let ext_opt = fpath.extension();
            match ext_opt {
                Some(ext) => {
                    match ext.to_str() {
                        Some("toml") => {
                            println!("TOML file \"{}\" ...", fpath.display());
                            let _json = convert_toml2json(content.unwrap());
                            let tmp_path_buf = fpath.with_extension("json");
                            let json_file_path = tmp_path_buf.as_path();
                            if json_file_path.exists() {
                                fs::remove_file(json_file_path).ok();
                            }
                            let mut json_file = match fs::File::create(json_file_path) {
                                Ok(x) => x,
                                Err(x) => { return Err(format!("Unable to create file \"{}\":{}", json_file_path.display(),x)); },
                            };
                            match json_file.write_all(_json.as_bytes()) {
                                Ok(()) => { println!("File \"{}\" produced", json_file_path.display()); },
                                Err(x) => { return Err(format!("Unable to write file \"{}\":{}", json_file_path.display(),x)); },
                            }
                        },
                        Some("json") => {
                            println!("JSON file \"{}\" ...", fpath.display());
                            let _toml = convert_json2toml(content.unwrap());
                            let tmp_path_buf = fpath.with_extension("toml");
                            let toml_file_path = tmp_path_buf.as_path();
                            if toml_file_path.exists() {
                                fs::remove_file(toml_file_path).ok();
                            }
                            let mut toml_file = match fs::File::create(toml_file_path) {
                                Ok(x) => x,
                                Err(x) => { return Err(format!("Unable to create file \"{}\":{}", toml_file_path.display(),x)); },
                            };
                            match toml_file.write_all(_toml.as_bytes()) {
                                Ok(()) => { println!("File \"{}\" produced", toml_file_path.display()); },
                                Err(x) => { return Err(format!("Unable to write file \"{}\":{}", toml_file_path.display(),x)); },
                            }
                        },
                        None => (),
                        _ => ()
                    }
                },
                None => {
                    return Err(format!("No extension for file \"{}\"", fpath.display()));
                },
            }
        },
        None => (),
    }
    return Ok(());
}

fn parse_list(files: Vec<String>) -> std::result::Result<(), String> {
    // disable multiple file processing
    if files.len() < 1 {
        println!("Usage: json2toml file1");
        return Err("You have to specify only one file to convert".to_string());
    }
    for file_name in files {
        parse_file(file_name.as_str().as_ref()).map_err(|x| { log_error(format!("file \"{}\" can't be parsed ({})", file_name, x)) }).ok();
    }
    return Ok(());
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    std::process::exit(match parse_list(args) {
        Ok(_) => 0,
        Err(err) => {
            log_error(format!("error: {}", err));
            1
        }
    });
}
