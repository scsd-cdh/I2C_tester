use serialport::SerialPort;

//https://github.com/flyaruu/rust-uart/blob/main/src/main.rs
pub struct Controller {
    port: Box<dyn SerialPort>,
    buffer: [u8; 8],
}

impl Controller {
    fn new(port: Box<dyn SerialPort>) -> Self {
        Controller {
            port,
            buffer: [0_u8; 8],
        }
    }
    // commands
}
