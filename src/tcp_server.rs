use std::{io, thread};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use errors_and_something_else::ErrorType;
use crate::server_handler::ServerHandler;

pub struct TcpServer {
    config: TcpServerConfig,
    f: Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, ErrorType> + Send + 'static>
}

impl TcpServer {
    pub(crate) fn new(f: Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, ErrorType> + Send + 'static>, config: TcpServerConfig) -> TcpServer {
        TcpServer {
            f,
            config,
        }
    }

    pub fn work(config: TcpServerConfig, handler: Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, ErrorType> + Send + 'static>) -> io::Result<()> {
        let socket_address = SocketAddr::new(IpAddr::from(config.ipv4addr), config.port);
        let listener = TcpListener::bind(socket_address)?;

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
                    let mut buffer = Vec::new();
                    let mut byte = [0; 1];
                    loop {
                        match stream.read(&mut byte) {
                            Ok(0) => {
                                println!("Connection closed by client");
                                break;
                            }
                            Ok(_) => {
                                buffer.push(byte[0]);
                                if byte[0] == b'\n' {
                                    let data_to_send = (handler)(buffer.clone());
                                    let d = match data_to_send {
                                        Ok(d) => d,
                                        Err(e) => Vec::from(e.bytes())
                                    };
                                    if let Err(e) = stream.write_all(&*d) {
                                        eprintln!("Failed to send data to client: {}", e);
                                        break;
                                    }
                                    buffer.clear();
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read data from client: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to establish connection: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn start_server(self) {
        let config = self.config.clone();
        let handler = self.f;
        let t = thread::spawn(move || {
            if let Err(e) = TcpServer::work(config, handler) {
                eprintln!("Server encountered an error: {}", e);
            }
        });

        match t.join() {
            Ok(_) => println!("Thread finished successfully"),
            Err(e) => println!("Thread encountered an error: {:?}", e),
        }
    }
}


#[derive(Clone)]
pub struct TcpServerConfig {
    pub(crate) ipv4addr: Ipv4Addr,
    pub(crate) port: u16,
}


