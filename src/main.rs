use opencv::{
    core,
    imgcodecs,
    imgproc,
    prelude::*,
    Result,
};
use std::io::Result;

fn main() -> Result<()> { 

    let img = imgcodecs::imread("photo.jpg", imgcodecs::IM_COLOR);

    Ok(())
}
