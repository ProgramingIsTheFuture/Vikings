use std::{
    io::{Read, Write},
    net::{TcpStream, UdpSocket},
};

use serde::Serialize;

pub struct Network {
    socket: UdpSocket,
}

pub enum Error {
    FailConnectTCP,
    AuthFailed,
    FailBindUDP,
    UDPSocketErr,
}

impl Network {
    pub fn new(auth: String) -> Result<Self, Error> {
        // Server socket
        Self::tcp_auth(auth)?;

        let socket = match UdpSocket::bind("127.0.0.1:9997") {
            Ok(s) => s,
            Err(_) => return Err(Error::FailBindUDP),
        };

        if socket.set_nonblocking(true).is_err() {
            return Err(Error::UDPSocketErr);
        };

        Ok(Self { socket })
    }

    fn tcp_auth(auth: String) -> Result<(), Error> {
        let mut tcp_socket = match TcpStream::connect("127.0.0.1:9998") {
            Ok(s) => s,
            Err(_) => return Err(Error::FailConnectTCP),
        };

        if tcp_socket.set_nonblocking(false).is_err() {
            return Err(Error::FailConnectTCP);
        }

        #[derive(Serialize)]
        struct Data {
            port: String,
            data: String,
        }

        let data = serde_json::to_string(&Data {
            port: "9997".to_string(),
            data: auth,
        })
        .unwrap();

        if tcp_socket.write(data.as_bytes()).is_err() {
            return Err(Error::AuthFailed);
        }

        let mut buf = [0; 10];
        match tcp_socket.read(&mut buf) {
            Ok(n) => if n == 0 {
                return Err(Error::AuthFailed)
            },
            Err(_) => return Err(Error::AuthFailed),
        };

        println!("[{}]!", String::from_utf8_lossy(&buf));

        Ok(())
    }

    pub fn read(&self) {
        let mut buffer = [0; 1024];
        // println!("Reading Server!");
        if let Ok(n) = self.socket.recv(&mut buffer) {
            // println!("Server: [{}]", String::from_utf8_lossy(&buffer[..n]))
        };
    }

    /*
    *let mut buf = [0; 1024];
           if let Ok(v) = s.recv(&mut buf) {
               let b = &mut buf[..v];
               println!("{}", String::from_utf8_lossy(b));
           };
    */

    fn network(&mut self) -> UdpSocket {
        println!("Init socket");
        let socket = UdpSocket::bind("127.0.0.1:9997").expect("Network problem!");

        println!("Non blocking");
        socket
            .set_nonblocking(true)
            .expect("Nonblocking network raise an error!");

        println!("Send");
        socket
            .send_to(String::from("Hello World!").as_bytes(), "127.0.0.1:9999")
            .unwrap();

        socket
    }
}
