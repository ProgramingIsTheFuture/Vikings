use std::net::UdpSocket;

pub struct Network {}

impl Network {
    pub fn new() -> Self {
        Self {}
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
        let socket = UdpSocket::bind("127.0.0.1:9998").expect("Network problem!");

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
