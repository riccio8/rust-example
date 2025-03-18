// importing modules
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// function for handle client

fn handle_client(mut stream: TcpStream){
    // buffer for reading data from client
    let mut data = [0; 1024];
    // read from the stream and save in the data buff
    stream.read(&mut data).
        expect("Failed to read data from the client");
    // convert data recived (as bytes) into string
    let re = String::from_utf8_lossy(&data[..]);

    println!("recived data:\n {}", re);
    
    //lets send back a message
    let response = "my message".as_bytes();
    stream.write(response).
        expect("Failed while responding to the client");
}

fn main(){
    let listen = TcpListener::bind("127.0.0.1:8080").
        expect("Failed binding to the address 127.0.0.1 at port 8080");
    println!("Server listening on port 127.0.0.1:8080");
    let mut incoming = 0;    
    for stream in listen.incoming() {
        match stream{
            Ok(stream) => {
                incoming += 1;
                if incoming >= 10 {
                    break;
                }

                thread::spawn ( ||{
                handle_client(stream);
                 });
            }
            
                Err(ref e) => {
                       eprintln!("Error listening with listener {:?}, Error: {}", stream, e);
                    //std error output
            }
         }
    }
    drop(listen);
}
