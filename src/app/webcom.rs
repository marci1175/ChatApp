
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
const MSG_SIZE: usize = 4096;
pub struct TcpClient {
    pub client: TcpStream,
    pub shutdown_requested: Arc<AtomicBool>,
    pub is_connected: bool,

    //pub joinhandl: JoinHandle<()>,
}

impl TcpClient {
    pub fn new(address: &str) -> io::Result<Self> {
        let client: TcpStream = TcpStream::connect(address)?;
        client.set_nonblocking(true).expect("failed to initiate non-blocking");
        let shutdown_requested = Arc::new(AtomicBool::new(false));
        let is_connected: bool = true;
        Ok(Self { client, shutdown_requested, is_connected})
    }

    pub fn send_message(&mut self, message: String) -> io::Result<()> {
        if self.is_connected == true && !(message.trim().len() == 0) {
            let (tx, rx) = mpsc::channel::<String>();
            let msg = message.trim().to_string();
            tx.send(msg).expect("Failed to send msg");

            
            match rx.try_recv() {
                Ok(msg) => {
                    let mut buff = msg.clone().to_owned().into_bytes();
                    buff.resize(4096, 0);
                    
                    self.client.write_all(&buff).expect("writing to socket failed");
                }, 
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => ()
            }
        }
        else {
            //do nothing
            
        }
        return Ok(());
    }

    pub fn shutdown(&self) -> io::Result<()> {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        self.client.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
    pub fn listen_for_msg(&mut self) -> String {
        let mut message_string: String = Default::default();
        let mut buff = vec![0; MSG_SIZE];
        match self.client.read_exact(&mut buff) {
            Ok(_) => {
                let msg: Vec<u8> = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let message : Vec<char> = String::from_utf8(msg)
                    .unwrap()
                    .chars()
                    .collect();
                message_string = message.iter().collect();
                message_string = message_string.trim().to_owned();
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                
            }
        }
        return message_string;
    }

        // Return a clone of the current state of the struct
    }
    

impl Default for TcpClient {
    fn default() -> Self {
        Self {
            client: TcpStream::connect("192.168.0.1:80".to_owned()).expect("Couldnt connect to ip"),
            shutdown_requested: AtomicBool::default().into(),
            is_connected: false,
        }
    }
}
