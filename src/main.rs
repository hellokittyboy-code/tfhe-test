use clap::{Arg, Command};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use chrono::Local;

fn main() -> Result<(), std::io::Error> {
    println!("Test function executed");
    test_process();
    Ok(())
}

#[cfg(test)]
fn main_test() {
    println!("Test function executed");
    test_process();
}

use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8, ClientKey, CompressedServerKey};
fn test_process() -> Result<(), Box<dyn std::error::Error>> {
    let mut now = Local::now();
    println!("[{}] start process init...", now.format("%Y-%m-%d %H:%M:%S%.3f"));
    // Basic configuration to use homomorphic integers
    //let config = ConfigBuilder::default().build();

    let config = ConfigBuilder::default().build();

    let (client_key, server_keys) = generate_keys(config);

    // Key generation
   // let (client_key, server_keys) = generate_keys(config);

    let clear_a = 1344u32;
    let clear_b = 5u32;
    let clear_c = 7u8;

    // Encrypting the input data using the (private) client_key
    // FheUint32: Encrypted equivalent to u32
    let mut encrypted_a = FheUint32::try_encrypt(clear_a, &client_key)?;
    let encrypted_b = FheUint32::try_encrypt(clear_b, &client_key)?;

    // FheUint8: Encrypted equivalent to u8
    let encrypted_c = FheUint8::try_encrypt(clear_c, &client_key)?;

    // On the server side:
    set_server_key(server_keys);

    now = Local::now();
    println!("[{}] set process server key...", now.format("%Y-%m-%d %H:%M:%S%.3f"));


    // Clear equivalent computations: 1344 * 5 = 6720
    let encrypted_res_mul = &encrypted_a * &encrypted_b;
    let now = Local::now();
    println!("[{}] set process sClear equivalent computations: 1344 * 5 = 6720...", now.format("%Y-%m-%d %H:%M:%S%.3f"));

    // Clear equivalent computations: 6720 >> 5 = 210
    encrypted_a = &encrypted_res_mul >> &encrypted_b;
    let now = Local::now();
    println!("[{}] set process Clear equivalent computations: 6720 >> 5 = 210", now.format("%Y-%m-%d %H:%M:%S%.3f"));

    // Clear equivalent computations: let casted_a = a as u8;
    let casted_a: FheUint8 = encrypted_a.cast_into();
    let now = Local::now();
    println!("[{}] set process Clear equivalent computations: let casted_a = a as u8;", now.format("%Y-%m-%d %H:%M:%S%.3f"));

    // Clear equivalent computations: min(210, 7) = 7
    let encrypted_res_min = &casted_a.min(&encrypted_c);
    let now = Local::now();
    println!("[{}] Clear equivalent computations: min(210, 7) = 7", now.format("%Y-%m-%d %H:%M:%S%.3f"));


    // Operation between clear and encrypted data:
    // Clear equivalent computations: 7 & 1 = 1
    let encrypted_res = encrypted_res_min & 1_u8;

    let now = Local::now();
    println!("[{}] encrpted res...", now.format("%Y-%m-%d %H:%M:%S%.3f"));


    // Decrypting on the client side:
    let clear_res: u8 = encrypted_res.decrypt(&client_key);
    assert_eq!(clear_res, 1_u8);
    let now = Local::now();
    println!("[{}] test process executed successfully", now.format("%Y-%m-%d %H:%M:%S%.3f"));
    Ok(())
}