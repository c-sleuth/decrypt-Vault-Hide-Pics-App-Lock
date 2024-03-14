# decrypt-Vault-Hide-Pics-App-Lock

This tool is intended for the use of decrypting the files within the apk Vault - Hide Pics, App Lock (com.netqin.ps)

## How It Works

The first 128 bytes of the files are XOR'd with 0x42 (66), this tool will reverse that process returning the file in its original state

## How to use

Use the pre-built binaries or compile from source.

Located within the directory "other_scripts" there is a script for decrypting single files at a time

```bash
git clone https://github.com/c-sleuth/decrypt-Vault-Hide-Pics-App-Lock.git
cargo build --release
```

### Decrypting
1. Make an output directory for all the decrypted files
2. Located this path on the device ```/mnt/sdcard/Android/data/com.netqin.ps/files/Documents/SystemAndroid/Data/{base64 encoded value}/.image``` this will be the input directory
3. Run the following command ```bash vault_decrypt <input dir> <output dir>```




 
