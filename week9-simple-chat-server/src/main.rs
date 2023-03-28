use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<usize, TcpStream>>>, id: usize) {
    loop {
        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let message = format!(
                    "Client {}: {}",
                    id,
                    String::from_utf8_lossy(&buf[..bytes_read])
                );
                println!("{}", message);

                let mut clients = clients.lock().unwrap();
                for (client_id, client) in clients.iter_mut() {
                    if *client_id != id {
                        if let Err(err) = client.write_all(message.as_bytes()) {
                            eprintln!("Error sending message to client {}: {}", client_id, err);
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading from client {}: {}", id, err);
                break;
            }
        }
    }

    let mut clients = clients.lock().unwrap();
    clients.remove(&id);
    println!("Client {} disconnected", id);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let mut next_id = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                let id = next_id;
                next_id += 1;

                thread::spawn(move || {
                    println!("Client {} connected", id);
                    {
                        let mut clients = clients.lock().unwrap();
                        clients.insert(id, stream.try_clone().unwrap());
                    }
                    handle_client(stream, clients, id);
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}
