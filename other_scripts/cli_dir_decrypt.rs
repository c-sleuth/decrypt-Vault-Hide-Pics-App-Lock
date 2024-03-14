use std::fs::{File, self};
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;

fn iter_dir(enc_dir: &str, out_dir: &str) -> io::Result<()> {
    if !is_valid_path(enc_dir) || !is_valid_path(out_dir) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path is not valid"));
    }
    let entries = fs::read_dir(enc_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let out_path = Path::new(out_dir).join(path.file_name().unwrap());
        decrypt_file(&path, &out_path)?;
    }
    Ok(())
}

fn decrypt_file(filename: &Path, outpath: &Path) -> io::Result<()> {
    let mut fp = File::open(filename)?;
    let mut buf = vec![0; 128];
    let bytes_read = fp.read(&mut buf)?;
    buf.resize(bytes_read, 0);
    println!("buf {:?}", buf);
    let xor_buf: Vec<u8> = buf.iter().map(|&b| b ^ 0x42).collect();
    let mut rest_of_file = Vec::new();
    fp.read_to_end(&mut rest_of_file)?;
    let mut output = File::create(outpath)?;
    output.write_all(&xor_buf)?;
    output.write_all(&rest_of_file)?;
    Ok(())
}

fn is_valid_path(path: &str) -> bool {
    !path.trim().is_empty() && std::path::Path::new(path).exists()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_dir> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let input_dir = &args[1];
    let output_dir = &args[2];

    iter_dir(input_dir, output_dir)?;
    Ok(())
}

