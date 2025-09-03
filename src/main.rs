#![allow(unused)]

use opencv::{
    core,
    imgcodecs,
    imgproc,
    prelude::*,
    Result,
};
use std::io::Result as IoResult;
use std::io::{
    Error,
    ErrorKind,
};
use std::sync::{Mutex, Arc};
use std::thread;

mod flag;
use flag::{Flags, Options};

fn main() -> IoResult<()> { 
    let mut args: Flags = Flags::new();
    args.parse()?;
    args.info();

    let mut files: Arc<Vec<String>> = Arc::new(args.files.clone());
    let img_mats: Arc<Mutex<Vec<core::Mat>>> = Arc::new(Mutex::new(Vec::new()));

    multi_read_files(files.len(), files.clone(), img_mats.clone())?;
   


    if args.contains(Options::NewFile) {
        let mut new_files = args.files;
        for file in &mut new_files {
            if file.chars().nth(0).unwrap() == 'Q' {
                file.insert(0, 'Z');
                continue;
            }
            file.insert(0, 'Q');
        }
        files = Arc::new(new_files);
    }
    multi_write_file(files.len(),  files.clone(), img_mats.clone());
    Ok(())
}

fn read_file_single(index: usize, files: Arc<Vec<String>>) -> IoResult<core::Mat> {
    let buf: core::Mat = imgcodecs::imread(&files[index], imgcodecs::IMREAD_COLOR).unwrap();
    if buf.empty() {
        eprintln!("Failed to open file {}", &files[index]);
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid File Type"));
    }
    Ok(buf)
}

fn multi_read_files(file_count: usize, files: Arc<Vec<String>>, img_mats: Arc<Mutex<Vec<core::Mat>>>) -> IoResult<()> {
    if file_count == 1 {
        let buffer = read_file_single(0, files.clone())?;
        img_mats.lock().unwrap().push(buffer);
        return Ok(());
    }

    let first_half_end = file_count / 2;

    thread::scope(|s| {
        let files_clone = Arc::clone(&files);
        let files_clone2 = Arc::clone(&files);
        let img_mat_clone = Arc::clone(&img_mats);
        let img_mat_clone2 = Arc::clone(&img_mats);

        s.spawn(move || {
            for i in 0..first_half_end {
                let matrix = read_file_single(i, files_clone.clone());
                match matrix {
                    Ok(val) => img_mat_clone.lock().unwrap().push(val),
                    Err(e) => eprintln!("Warn! Invalid File {} Error {}, skipping...", files_clone[i], e),
                }
            }
        });

        s.spawn(move || {
            for i in first_half_end..file_count {
                let matrix = read_file_single(i, files_clone2.clone());
                match matrix {
                    Ok(val) => img_mat_clone2.lock().unwrap().push(val),
                    Err(e) => eprintln!("Warn! Invalid File {} Error {}, skipping...", files_clone2[i], e),
                }
            }
        });

    });

    Ok(())
}


fn multi_write_file(file_count: usize, files: Arc<Vec<String>>, img_mats: Arc<Mutex<Vec<core::Mat>>>) {
    if file_count == 1 {
        imgcodecs::imwrite(&files[0], &img_mats.lock().unwrap()[0], &core::Vector::<i32>::new()).unwrap();
        return;
    }

    let first_half = file_count / 2;
    thread::scope(|s| {
        let first_files = Arc::clone(&files);
        let first_mats = Arc::clone(&img_mats);
        s.spawn(move || {
            for i in 0..first_half {
                imgcodecs::imwrite(&first_files[i], &first_mats.lock().unwrap()[i], &core::Vector::<i32>::new()).unwrap();
            }
        });

        let second_files = Arc::clone(&files);
        let second_mats = Arc::clone(&img_mats);
        s.spawn(move || {
            for i in first_half..file_count {
                imgcodecs::imwrite(&second_files[i], &second_mats.lock().unwrap()[i], &core::Vector::<i32>::new()).unwrap();
            }
        });

    });
}
