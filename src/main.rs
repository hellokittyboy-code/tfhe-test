use clap::{Arg, Command};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use chrono::Local;
use log::info;
use tfhe::ConfigBuilder;
use tfhe::generate_keys;
use tfhe::ClientKey;
use tfhe::CompressedServerKey;
use tfhe::FheUint32;
use tfhe::FheUint8;
use tfhe::set_server_key;
use tfhe::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut now = Local::now();
    info!("[{}] start process init...", now.format("%Y-%m-%d %H:%M:%S%.3f"));
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
    info!("[{}] set process server key...", now.format("%Y-%m-%d %H:%M:%S%.3f"));


    // Clear equivalent computations: 1344 * 5 = 6720
    let encrypted_res_mul = &encrypted_a * &encrypted_b;
    let now = Local::now();
    info!("[{}] set process sClear equivalent computations: 1344 * 5 = 6720...", now.format("%Y-%m-%d %H:%M:%S%.3f"));

    // Clear equivalent computations: 6720 >> 5 = 210
    encrypted_a = &encrypted_res_mul >> &encrypted_b;
    let now = Local::now();
    info!("[{}] set process Clear equivalent computations: 6720 >> 5 = 210", now.format("%Y-%m-%d %H:%M:%S%.3f"));

    // Clear equivalent computations: let casted_a = a as u8;
    let casted_a: FheUint8 = encrypted_a.cast_into();
    let now = Local::now();
    info!("[{}] set process Clear equivalent computations: let casted_a = a as u8;", now.format("%Y-%m-%d %H:%M:%S%.3f"));

    // Clear equivalent computations: min(210, 7) = 7
    let encrypted_res_min = &casted_a.min(&encrypted_c);
    let now = Local::now();
    info!("[{}] Clear equivalent computations: min(210, 7) = 7", now.format("%Y-%m-%d %H:%M:%S%.3f"));


    // Operation between clear and encrypted data:
    // Clear equivalent computations: 7 & 1 = 1
    let encrypted_res = encrypted_res_min & 1_u8;

    let now = Local::now();
    info!("[{}] encrpted res...", now.format("%Y-%m-%d %H:%M:%S%.3f"));


    // Decrypting on the client side:
    let clear_res: u8 = encrypted_res.decrypt(&client_key);
    assert_eq!(clear_res, 1_u8);
    let now = Local::now();
    info!("[{}] test process executed successfully", now.format("%Y-%m-%d %H:%M:%S%.3f"));
    Ok(())
}