
#include "libqimg.hxx"
#include <opencv2/highgui.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/opencv.hpp>

#ifndef __LIB__
int main(int argc, char** argv) { 
  int x = 1;
  std::vector<std::string> s;
  while (x < argc) {
    s.push_back(argv[x]);
    x++;
  }
  return QimgMain(image_mode::normal, s);
}
#endif

int QimgMain(image_mode mode, std::vector<std::string> paths) {
  for (auto &path : paths) {
    cv::Mat Image = cv::imread(path);
    if (Image.empty())
      return 1;
   
  }
  return 0;
}

