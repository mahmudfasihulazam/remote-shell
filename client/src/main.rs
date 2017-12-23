
use std::io;
use std::io::{BufReader, LineWriter};
use std::io::prelude::*;
use std::net::{TcpStream};
use std::process;

fn read_output(reader : &mut BufReader<&TcpStream>) {
}

fn main() {
    // Establish TCP connection:
    let server_address = String::from("127.0.0.1:60000");
    println!("Address of server: {}", server_address);
    let stream = TcpStream::connect(server_address)
        .expect("Could not connect to server");

    // Setup stream readers and writers:
    let mut reader : BufReader<&TcpStream> = BufReader::new(&stream);
    let mut writer : LineWriter<&TcpStream> = LineWriter::new(&stream);

    // Continuously get commands from stdin and send them to server:
    loop {
        let mut comm = String::new();
        let mut buf = String::new();
unsafe {
        // Read output from Server:
        match reader.read_until(0, buf.as_mut_vec()) {
            Ok(0) => {
                process::exit(0);
            },
            Ok(_) => {
                ;
            },
            Err(_) => {
                println!("Error");
                continue;
            },
        }

        println!("{}", buf);
}

        // Send command to server:
        io::stdin().read_line(&mut comm)
            .expect("Could not read from input");
        writer.write_all(format!("{}", comm).as_bytes())
            .expect("Failed to write to server");
        writer.flush().expect("");
    }
}

