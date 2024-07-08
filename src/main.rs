use std::time::Duration;

use rppal::uart::{Parity, Queue, Uart};

fn main() {
    // Create a new UART instance
    let mut uart = Uart::new(9600, Parity::None, 8, 1).unwrap();

    // Set the read and write mode
    uart.set_write_mode(true).unwrap();
    uart.set_read_mode(4, Duration::from_secs(1)).unwrap();

    // Clear the buffer
    uart.flush(Queue::Both).unwrap();

    // Send a integer
    let mut request_buffer = [0; 9];

    // Order type
    request_buffer[0] = 0xB1;

    // 32-bit integer as 8 bytes
    let number: i32 = 1000000;
    request_buffer[1] = (number >> 24) as u8;
    request_buffer[2] = (number >> 16) as u8;
    request_buffer[3] = (number >> 8) as u8;
    request_buffer[4] = number as u8;

    // Registration number
    request_buffer[5] = 0;
    request_buffer[6] = 0;
    request_buffer[7] = 8;
    request_buffer[8] = 6;

    // Send the buffer
    uart.write(&request_buffer).unwrap();

    // Read the response
    let mut response_buffer = [0; 4];
    uart.read(&mut response_buffer).unwrap();

    // Print the response
    println!("Response: {:?}", i32::from_be_bytes(response_buffer));

    // Clear the buffer
    uart.flush(Queue::Both).unwrap();
}
