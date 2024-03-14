use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn decrypt_file(filename: &Path) -> io::Result<()> {
    let mut fp = File::open(filename)?;
    
    let mut buf = vec![0; 64];
    fp.read_exact(&mut buf)?;
    //let bytes_read = fp.read(&mut buf);
    //buf.resize(bytes_read?, 0); // buf currently has the encrypted data
    let xor_buf: Vec<u8> = buf.iter().map(|&b| b ^ 0x42).collect();
    
    let mut rest_of_file = Vec::new();
    fp.read_to_end(&mut rest_of_file)?;
    
    let mut output = File::create(filename.file_name().unwrap())?;
    output.write_all(&xor_buf)?;
    output.write_all(&rest_of_file)?;
   Ok(()) 
}

fn main() {
    let filename = Path::new( /* file path here */ );
    //let filename = "C:/Users/win10/Documents/vault/imgs/1710421248768.bin";
    match decrypt_file(filename) {
        Ok(bytes) => println!("{:?}", bytes),
        Err(e) => eprint!("Error: {:?}", e),
    }
}

