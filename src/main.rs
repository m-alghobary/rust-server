use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

const SERVER_ADDRESS: &str = "localhost:7070";

fn main() -> std::io::Result<()> {
    let listener = match TcpListener::bind(SERVER_ADDRESS) {
        Ok(listener) => listener,
        Err(_) => panic!("Failed to listen on {}!", SERVER_ADDRESS),
    };

    println!("Server is listening on {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream)?,
            Err(_) => eprintln!("A failed connection!"),
        }
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    println!(
        "connection estaplished from {}",
        stream.local_addr().unwrap().ip()
    );

    let mut writer = BufWriter::new(stream.try_clone()?);
    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    _ = reader.read_line(&mut line)?;

    println!("Received message, {}", line);

    thread::sleep(Duration::from_secs(3));

    writer.write_all(line.as_bytes())?;

    println!("Received message, {} back", line);

    Ok(())
}
