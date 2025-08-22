
#include "libqimg.hxx"
#include <opencv2/core.hpp>

#ifndef __LIB__
int main(int argc, char** argv) { 
  int x = 0;
  std::vector<std::string> s;
  while (x < argc) {
    s.push_back(argv[x]);
    x++;
  }
  return libqimg_file_entry(image_mode::normal, s);
}
#endif

int libqimg_file_entry(image_mode mode, std::vector<std::string> paths) {
  for (auto &path : paths) {
    
  }
  return 0;
}

