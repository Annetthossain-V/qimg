use opencv::{
    core::{self, AlgorithmHint, MatExprTrait, MatTrait, Scalar},
    imgcodecs, imgproc,
    prelude::*,
    Result,
};

use std::io::Result as IoResult;
use std::sync::{Arc, Mutex};

pub fn img_saturate_builtin(img_mats: Arc<Mutex<Vec<core::Mat>>>) -> IoResult<()> {
    let mut mats = img_mats.lock().unwrap();
    let power: f32 = 1.3; // make this dynamic env variable
    if mats.len() == 1 {
        img_saturate_builtin_single(&mut mats[0], 1.4)?;
        return Ok(());
    }

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
