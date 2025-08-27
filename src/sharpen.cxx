
#include "sharpen.hpp"
#include <opencv2/opencv.hpp>

void ImageSharpenBasic(cv::Mat& Img, float Percent, cv::Size BlurKSize) {
  cv::Mat blurred;
  cv::Mat HighFreq;

  cv::GaussianBlur(Img, blurred, BlurKSize, 0);
  cv::subtract(Img, blurred, HighFreq);
  cv::addWeighted(Img, 1.0, HighFreq, Percent, 0, Img);

  return;
}
