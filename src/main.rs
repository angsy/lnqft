use std::ffi::OsStr;
use std::fs;
use std::io::{prelude::*, stdin};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn file_handler() -> (String, String) {

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

    let mut _file_data: String = String::new();
    match fs::read_to_string(Path::new(&_file_path)) {
        Ok(data) => {
            _file_data = data;
        },
        Err(error) => {
            println!("{0}", error);
            println!("Program will terminate automatically......");
            sleep(Duration::from_secs(5));
            exit(0);
        }
    };

    return (_file_name, _file_data);
}

fn socket_address() -> String {

    let ipv4_octets: Vec<u8> = loop {

        println!("IP v4 address to listen to...");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut verified: bool = true;

        let unparsed: Vec<&str> = input.strip_suffix("\r\n").unwrap().split(".").collect();
        let mut parsed: Vec<u8> = Vec::new();
        for data in unparsed {
            match data.parse::<u8>() {
                Ok(integer) => {
                    parsed.push(integer);
                },
                Err(_) => {
                    verified = false;
                }
            };
        };

        if parsed.len() != 4 || verified == false {
            println!("\x1B[31;1mError in handling the specified address. Please try again.\x1B[0m");
        } else {
            break parsed;
        };
    };

    let port_number: u16 = loop {

        println!("Port number to listen to...");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        let input: String = String::from(input.strip_suffix("\r\n").unwrap());

        let parsed: u16;
        match input.parse::<u16>() {
            Ok(integer) => {
                parsed = integer;
                break parsed;
            },
            Err(_) => {
                println!("\x1B[31;1mError in handling the specified port number. Please try again.\x1B[0m");
            }
        };
    };

    return format!("{0}.{1}.{2}.{3}:{4}", ipv4_octets[0], ipv4_octets[1], ipv4_octets[2], ipv4_octets[3], port_number);
}

fn connection_handler(mut connection: TcpStream, file_name: &String, file_data: &String) {

    let expected_request_uri: String = format!("/{0}", file_name);

    let mut receiving_buffer: [u8; 1024] = [0; 1024];
    connection.read(&mut receiving_buffer).unwrap();

    let request: String = String::from_utf8_lossy(&receiving_buffer).to_string();
    let request: Vec<&str> = request.split("\r\n").collect();

    let request_line: Vec<&str> = request[0].split(" ").collect();

    if request_line[0] == "GET" && request_line[1] == expected_request_uri {
        let outgoing_data: String = format!(
            "{0} 200 OK\r\nContent-Disposition: attachment; filename={1}\r\nContent-Length: {2}\r\n\r\n{3}",
            request_line[2],
            file_name,
            file_data.len(),
            file_data
        );
        connection.write(outgoing_data.as_bytes()).unwrap();
        connection.flush().unwrap();
    };
}

fn main() {

    let (file_name, file_data) = file_handler();

    let address: String = socket_address();

    let link: String = format!("{0}/{1}", &address, &file_name);
    println!("\nFile will be temporarily hosted at...\n\x1B[32;1m{0}\x1B[0m\nPlease exits the program after transfer completed.\n", link);

    let binded_tcplistener: TcpListener;
    match TcpListener::bind(address) {
        Ok(tcplistener) => {
            binded_tcplistener = tcplistener;
        },
        Err(error) => {
            println!("{0}", error);
            println!("Program will terminate automatically......");
            sleep(Duration::from_secs(5));
            exit(0);
        }
    };

    for incoming_connection in binded_tcplistener.incoming() {
        match incoming_connection {
            Ok(stream) => {
                connection_handler(stream, &file_name, &file_data);
            },
            Err(error) => {
                println!("{0}", error);
                println!("Program will terminate automatically......");
                sleep(Duration::from_secs(5));
                exit(0);
            }
        };
    };
}
