use std::fmt::format;
use std::io::{Read, Write};
use std::net::TcpStream;
// use std::str::from_utf8;
use std::time::Duration;

fn main() {
    match TcpStream::connect("localhost:4050") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 4050");

            let msg = b"GET /chat HTTP/1.1\r\n\
                Host: localhost\r\n\
                Upgrade: websocket\r\n\
                Connection: Upgrade\r\n\
                Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
                Origin: http://localhost.com\r\n\
                Sec-WebSocket-Protocol: ws\r\n\
                Sec-WebSocket-Version: 13\r\n\
                \r\n";

            stream
                .set_read_timeout(Some(Duration::from_secs(1)))
                .expect("TODO: panic message");

            stream.write(msg).expect("Sent data");
            stream.flush().expect("Flush error");
            // println!("Sent Hello, awaiting reply...");

            // let mut data = [0u8; 1024]; // using 6 byte buffer
            let mut data: String = String::new();

            // loop {
            //     let mut s = String::new();
            //     println!("{:#?}", stream.read_to_string(&mut s).unwrap())
            // }

            loop {
                // match stream.read(&mut data) {
                //     Ok(_) => {
                //         // let text = from_utf8(&data).unwrap();
                //         // print!("{:?}", &data);
                // 
                //         // let mut s = String::new();
                //         // for c in &data {
                //         //     s += &format!("0{:b}", c);
                //         // }
                //         // println!("Data: {}", s);
                //         // println!("New try for data: {:?}", String::from_utf8((data).to_vec()));
                //         // println!("Unexpected reply: {}", text);
                // 
                //         println!("{}", String::from_utf8(Vec::from(data)).unwrap_or(String::new()));
                //         data = [0u8; 1024];
                //     }
                //     Err(e) => {
                //         println!("Failed to receive data: {}", e)
                //     }
                // }
                
                match stream.read_to_string(&mut data) {
                    Ok(_) => {
                        println!("Data: {}", data);
                    }
                    Err(e) => {
                        println!("Error: {:#?}", e)
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
