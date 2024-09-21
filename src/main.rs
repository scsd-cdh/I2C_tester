use std::{
    io::{self, stdout, Write},
    thread::{self, sleep, Thread},
    time::Duration,
};

use serial::Device;
use serialport::SerialPort;

mod serial;

const PORT_PATH: &str = "/dev/ttyACM0";
const MESSAGE1: &str = "first";
const MESSAGE2: &str = "second";
const MESSAGE3: &str = "third";
const MESSAGE4: &str = "fourth";

fn main() {
    let mut device = Device::new(PORT_PATH.to_string(), 9600);
    device.write(MESSAGE1);
    device.read();
    device.write(MESSAGE2);
    device.read();
    device.write(MESSAGE3);
    device.read();
    device.write(MESSAGE4);
    device.read();

    // let builder = serialport::new(PORT_PATH, 9600);
    // println!("{:?}", &builder);
    // let mut port = builder.open().unwrap_or_else(|e| {
    //     eprintln!("Failed to open \"{}\". Error: {}", PORT_PATH, e);
    //     ::std::process::exit(1);
    // });
    //
    // thread::sleep(Duration::from_millis(5000)); // to have time to open SERIEAL MONITOR in
    //                                             // ARduino cuz we can't open port when it alrdy listens
    // write(&mut port, MESSAGE1);
    //
    // thread::sleep(Duration::from_millis(2000)); // to have time to open SERIEAL MONITOR in
    // read(&mut port);
    //
    // write(&mut port, "second");
    // thread::sleep(Duration::from_millis(2000)); // to have time to open SERIEAL MONITOR in
    // read(&mut port);
    //
    // write(&mut port, "third");
    // thread::sleep(Duration::from_millis(2000)); // to have time to open SERIEAL MONITOR in
    // read(&mut port);
    //
    // write(&mut port, "fourth");
    // thread::sleep(Duration::from_millis(2000)); // to have time to open SERIEAL MONITOR in
    // read(&mut port);
}

fn read(port: &mut Box<dyn SerialPort>) {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let mut buf_size: usize = 0;
    println!("Receiving data on {} at {} baud:", &PORT_PATH, 9600);
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                buf_size = t;
                io::stdout().write_all(&serial_buf[..t]).unwrap();
                io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }

        println!("buf size:{buf_size}");
        if buf_size == 0 {
            break;
        }
        buf_size = 0;
    }
}

fn write(port: &mut Box<dyn SerialPort>, message: &str) {
    println!("Writing '{}' to {} at {} baud", message, PORT_PATH, 9600);
    match port.write(message.as_bytes()) {
        Ok(_) => {
            port.flush().unwrap(); // Ensure the data is sent out before continuing
            println!("writing: {}", message);
            std::io::stdout().flush().unwrap();
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }
}
