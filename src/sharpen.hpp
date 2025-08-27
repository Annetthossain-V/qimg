#pragma once

#include <opencv2/core/mat.hpp>
#include <opencv2/opencv.hpp>

void ImageSharpenBasic(cv::Mat& Img, float Percent, cv::Size BlurKSize);
