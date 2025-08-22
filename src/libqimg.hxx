#pragma once
#include <vector>
#include <string>


enum image_mode {
  normal,
  night,
  hdr,
  high_light,
};


int QimgMain(image_mode mode, std::vector<std::string> paths);


