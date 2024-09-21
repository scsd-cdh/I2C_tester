use serialport::SerialPort;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Device {
    path: String,
    baud_rate: u32,
    port: Box<dyn SerialPort>,
}

impl Device {
    pub fn new(path: String, baud_rate: u32) -> Self {
        let port = Self::connect(&path, baud_rate);
        Self {
            path,
            baud_rate,
            port,
        }
    }

    pub fn connect(path: &String, baud_rate: u32) -> Box<dyn SerialPort> {
        let builder = serialport::new(path.clone(), baud_rate);
        println!("Connection SUCCESS!: {:?}", &builder);
        builder.open().unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", path, e);
            ::std::process::exit(1);
        })
    }

    pub fn read(&mut self) {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        let mut buf_size: usize = 0;
        println!(
            "Receiving data on {} at {} baud:",
            &self.path, self.baud_rate
        );

        // read until get something
        while buf_size == 0 {
            match self.port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    buf_size = t;
                    io::stdout().write_all(&serial_buf[..t]).unwrap();
                    io::stdout().flush().unwrap();
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => {
                    eprintln!("Error while reading: {:?}", e);
                    break;
                }
            }
            println!("DEBUG: no input");
        }
    }

    pub fn write(&mut self, message: &str) {
        println!(
            "Writing '{}' to {} at {} baud",
            message, self.path, self.baud_rate
        );
        match self.port.write(message.as_bytes()) {
            Ok(_) => {
                self.port.flush().unwrap(); // Ensure the data is sent out before continuing
                println!("writing: {}", message);
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
