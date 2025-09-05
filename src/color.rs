use opencv::{
    core::{self, AlgorithmHint, MatExprTrait, MatTrait, Scalar},
    imgcodecs, imgproc,
    prelude::*,
    Result,
};

use std::io::Result as IoResult;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn img_saturate_builtin(img_mats: Arc<Mutex<Vec<core::Mat>>>) -> IoResult<()> {
    let mut power: f32 = 1.4;
    let mat_len: usize = img_mats.lock().unwrap().len();

    let env_value = std::env::var("QIMG_BUILTIN_SATURATION_POW");
    match env_value {
        Ok(val) => {
            power = val.parse().expect("invalid value format");
        }
        Err(e) => {}
    }

    if mat_len == 1 {
        img_saturate_builtin_single(&mut img_mats.lock().unwrap()[0], power)?;
        return Ok(());
    }

    let part1 = mat_len / 2;
    thread::scope(|s| {
        let mats1 = Arc::clone(&img_mats);
        let mats2 = Arc::clone(&img_mats);

        s.spawn(move || {
            for i in 0..part1 {
                img_saturate_builtin_single(&mut mats1.lock().unwrap()[i], power).unwrap();
            }
        });
        s.spawn(move || {
            for i in part1..mat_len {
                img_saturate_builtin_single(&mut mats2.lock().unwrap()[i], power).unwrap();
            }
        });
    });

    Ok(())
}

fn img_saturate_builtin_single(img_mat: &mut core::Mat, n: f32) -> IoResult<()> {
    let mut hsv = Mat::default();
    imgproc::cvt_color(
        img_mat,
        &mut hsv,
        imgproc::COLOR_BGR2HSV,
        0,
        AlgorithmHint::ALGO_HINT_ACCURATE,
    )
    .unwrap();

    let mut s = Mat::default();
    core::extract_channel(&hsv, &mut s, 1).unwrap();

    let mut s_f = Mat::default();
    s.convert_to(&mut s_f, core::CV_32F, 1.0, 0.0).unwrap();

    let mut tmp = Mat::default();
    core::multiply(&s_f, &Scalar::all(n as f64), &mut tmp, 1.0, -1).unwrap();

    let mut tmp2 = Mat::default();
    core::min(&tmp, &Scalar::all(255.0), &mut tmp2).unwrap();
    tmp = tmp2;
    tmp.convert_to(&mut s, core::CV_8U, 1.0, 0.0).unwrap();

    core::insert_channel(&s, &mut hsv, 1).unwrap();
    let mut out = Mat::default();
    imgproc::cvt_color(
        &hsv,
        &mut out,
        imgproc::COLOR_HSV2BGR,
        0,
        AlgorithmHint::ALGO_HINT_ACCURATE,
    )
    .unwrap();

    *img_mat = out;
    Ok(())
}

pub fn multi_white_enhance(matrix: Arc<Mutex<Vec<core::Mat>>>) {
    let matrix_len = matrix.lock().unwrap().len();
    if matrix_len == 1 {
        white_enhance_single(&mut matrix.lock().unwrap()[0]);
        return;
    }

    let part1 = matrix_len / 2;
    thread::scope(|s| {
        let mat1 = Arc::clone(&matrix);
        let mat2 = Arc::clone(&matrix);

        s.spawn(move || {
            for i in 0..part1 {
                white_enhance_single(&mut mat1.lock().unwrap()[i]);
            }
        });
        s.spawn(move || {
            for i in part1..matrix_len {
                white_enhance_single(&mut mat2.lock().unwrap()[i]);
            }
        });
    });
}

fn white_enhance_single(matrix: &mut core::Mat) {
    assert_eq!(matrix.typ(), core::CV_8UC3);

    let rows = matrix.rows();
    let cols = matrix.cols();

    for y in 0..rows {
        for x in 0..cols {
            let pixel = *matrix.at_2d::<core::Vec3b>(y, x).unwrap();
            // pixel format bgr

            println!("blue value: {}", pixel[0]);
        }
    }
}
