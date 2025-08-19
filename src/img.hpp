#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "libqimg.hxx"

class img {
public:
  std::string path;
  std::vector<image_mode> modes;

  void retrive_data();
  void write_data();
private:
  std::vector<std::uint8_t> data;

protected:

};
