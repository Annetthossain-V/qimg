
#include "color.hpp"
#include <opencv2/opencv.hpp>

void ImgSaturateBasic(cv::Mat &Img, float SaturateScale) {
  if (SaturateScale > 255)
    return;

  cv::Mat hsv;
  cv::cvtColor(Img, hsv, cv::COLOR_BGR2HSV);

  std::vector<cv::Mat> hsvChannels;
  cv::split(hsv, hsvChannels);

  hsvChannels[1].convertTo(hsvChannels[1], CV_32F);
  hsvChannels[1] *= SaturateScale;
  hsvChannels[1] = cv::min(hsvChannels[1], 255.0f);

  hsvChannels[1].convertTo(hsvChannels[1], CV_8U);
  
  cv::merge(hsvChannels, hsv);
  cv::cvtColor(hsv, Img, cv::COLOR_HSV2BGR);
  return;
}

void BalanceWhiteSAdv(cv::Mat &Img) {

  return;
}
