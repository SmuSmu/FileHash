use sha3::{Digest, Sha3_256};

fn main() {
    let read = fileread("test.txt");
    match read
        {
        Ok(v) => println!("Done: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
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