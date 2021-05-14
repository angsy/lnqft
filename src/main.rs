use std::ffi::OsStr;
use std::fs;
use std::io::stdin;
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn file_handler() -> (String, String, Vec<u8>) {

    let mut _file_path: String = String::new();
    loop {
        println!("Full path to the file to be transferred...");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        let input: String = String::from(input.strip_suffix("\r\n").unwrap());
        if Path::new(&input).is_file() == true {
            _file_path = input;
            break
        } else {
            println!("\x1B[31;1mFile path specified invalid. Please try again.\x1B[0m");
        };
    };

    let mut _file_name: String = String::new();
    let file_name_osstr: &OsStr = Path::new(&_file_path).file_name().unwrap();
    _file_name = String::from(Path::new(&file_name_osstr).to_str().unwrap());

    let mut _file_data: Vec<u8> = Vec::new();
    match fs::read(Path::new(&_file_path)) {
        Ok(data) => _file_data = data,
        Err(error) => {
            println!("{0}", error);
            println!("Program will terminate automatically......");
            sleep(Duration::from_secs(5));
            exit(0);
        }
    };

    return (_file_path, _file_name, _file_data);
}

fn main() {
    let (file_path, file_name, file_data) = file_handler();

    println!("\nFOR DEBUGGING PURPOSE ONLY:\nfile_path => {}\nfile_name => {}\nfile_data => {:?}\n", file_path, file_name, file_data);
}
