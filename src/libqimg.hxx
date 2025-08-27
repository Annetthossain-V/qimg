#pragma once
#include <vector>
#include <string>


enum image_mode {
  normal,
  night,
  hdr,
  high_light,
};

struct ImgCodes {
  
} __attribute__((packed));


int QimgFileList(std::vector<image_mode> mode, std::vector<std::string> paths);
bool QimgMain(image_mode mode, std::string ImagePath);


