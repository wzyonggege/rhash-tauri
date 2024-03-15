use md5;
use std::time::Instant;
use sha2::{Digest, Sha256, Sha512};
use sha1::Sha1;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;
// use tauri::{command, Runtime};
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug)]
pub struct DigestResult {
    pub path: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
    pub md5: String,
    pub duration: f64,
}

impl DigestResult {
    pub fn print_result(&self) -> String {
        format!("
        <b>文件</b>: {}<br><br>
        用时: {:.3}秒<br><br>
        <b>MD5</b>: {}<br>
        <b>SHA-1</b>: {}<br>
        <b>SHA-256</b>: {}<br>
        <b>SHA-512</b>: {}<br>
        ", 
        self.path, self.duration, self.md5, self.sha1, self.sha256, self.sha512)
    }
}

/// calculates sha256 digest as lowercase hex string
pub fn cal_digest(path: &PathBuf) -> Result<DigestResult, String> {
    let start = Instant::now();

    let input = File::open(path).unwrap();
    let mut reader = BufReader::new(input);

    let mut sha1_hasher = Sha1::new();
    let mut sha256_hasher = Sha256::new();
    let mut sha512_hasher = Sha512::new();
    let mut md5_hasher = md5::Context::new();
    let mut buffer = [0; 1024*128];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        sha1_hasher.update(&buffer[..bytes_read]);
        sha256_hasher.update(&buffer[..bytes_read]);
        sha512_hasher.update(&buffer[..bytes_read]);
        md5_hasher.consume(&buffer[..bytes_read]);
    }
    let sha1_digest = sha1_hasher.finalize();
    let sha256_digest = sha256_hasher.finalize();
    let sha512_digest = sha512_hasher.finalize();
    let md5_digest = md5_hasher.compute();

    let duration = start.elapsed().as_secs_f64();
    let resp = DigestResult {
        path: path.as_path().display().to_string(),
        sha1: format!("{:x}", sha1_digest),
        sha256: format!("{:x}", sha256_digest),
        sha512: format!("{:x}", sha512_digest),
        md5: format!("{:x}", md5_digest),
        duration,
    };
    
    Ok(resp)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn my_custom_command(a: f32, b: f32) -> f32 {
    a * b 
}

#[tauri::command]
async fn dig_file(path: String) -> String {
    let path_buf = PathBuf::from_str(&path).unwrap();
    let dig_res = cal_digest(&path_buf).unwrap().print_result();
    dig_res
}

fn main() {
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command, dig_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
