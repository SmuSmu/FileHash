extern crate clap;


use sha3::{Digest, Sha3_256};

fn main() {
    let matches = clap::App::new("checksum")
        .version("0.1.0")
        .author("SmuSmu")
        .about("Calculate checksums for input file")
        .arg(
            clap::Arg::with_name("INPUT FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("algorithm")
                .short("a")
                .takes_value(true)
                .possible_values(&["MD5", "SHA1"])
                .default_value("SHA1"),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let _path = std::path::Path::new(matches.value_of("INPUT FILE").unwrap());

    let read = fileread("test.txt");
    match read
    {
        Ok(v) => println!("Done: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
        }


    match matches.value_of("algorithm").unwrap() {
        "MD5" => println!("{}",  1),
        "SHA1" => println!("{}",224),
        _ => unreachable!(),
    }
}

fn fileread(path:&str) -> std::io::Result<()>
    {
    use std::io::prelude::*;
    
    let mut f = std::fs::File::open(path)?;
    let mut buffer = [0; 1024];
    let mut hasher = Sha3_256::new();

    while let Ok(bytes_read) = f.read(&mut buffer)
        {
        if bytes_read == 0
            {
            break;
            }
        else
            {
            hasher.input(&buffer[..bytes_read]);
            }
        }
    // read hash digest
    let result = hasher.result();
    println!("Sha3_256 : {:X}", result);

    // Test
    //assert_eq!(format!("{:X}", result), "C362A3B1B03B5C5B9EB335EAD33FA05C77365302C0F12ECD6BC741FE4F792236");
    Ok(())
    }