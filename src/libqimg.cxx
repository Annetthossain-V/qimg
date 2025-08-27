
#include "libqimg.hxx"
#include "sharpen.hpp"
#include "color.hpp"
#include <opencv2/highgui.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/opencv.hpp>

#ifndef __LIB__
int main(int argc, char** argv) { 
  int x = 1;
  std::vector<std::string> s;
  std::vector<image_mode> m;
  while (x < argc) {
    s.push_back(argv[x]);
    m.push_back(image_mode::normal);
    x++;
  }
  return QimgFileList(m, s);
}
#endif

int QimgFileList(std::vector<image_mode> mode, std::vector<std::string> paths) {
  size_t index = 0;
  for (auto &path : paths) {
    if (QimgMain(mode[index], path) != true)
      return 1;
    index++;
  }
  return 0;
}

bool QimgMain(image_mode modes, std::string ImagePath) { 
  cv::Mat ImgMat = cv::imread(ImagePath, cv::IMREAD_COLOR);
  if (ImgMat.empty())
    return false;

  ImageSharpenBasic(ImgMat, 6, cv::Size(5, 5));
  ImgSaturateBasic(ImgMat, 2.4);

  BalanceWhiteSAdv(ImgMat);

  // make this dynamic
  std::string ImgName = "Qimg_Processed.jpg";
  if (!cv::imwrite(ImgName, ImgMat))
    return false;
  return 0;
}

