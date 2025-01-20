
#include <iostream>
#include <range/v3/all.hpp>
#include <vector>

uint64_t UncheckedDecode(uint8_t *buffer, int start_bit, int len) {
  auto start_byte = start_bit / 8;
  auto start_bit_inbyte = start_bit % 8;

  auto padding_size = (len + start_bit_inbyte);
  auto bytes = padding_size / 8 + (padding_size % 8 == 0 ? 0 : 1);
  if (bytes >= sizeof(uint64_t)) {
    return 0;
  }
  uint64_t result = 0;
  memcpy(&result, buffer + start_byte, bytes);
  // std::cout << std::bitset<64>(result) << std::endl;
  result <<= start_bit_inbyte;
  // std::cout << std::bitset<64>(result) << " " << std::endl;
  return result >> (8 * bytes - len);
}

namespace views = ranges::views;

int main() {
  uint8_t buf[8] = {0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f};
  for (auto &&i : buf) {
    std::cout << std::hex << static_cast<int>(i) << " ,";
  }
  std::cout << std::endl;

  auto result = views::ints(0, 33) | views::transform([&buf](auto v) {
                  return UncheckedDecode(buf, v, 8);
                });
  for (auto v : result) {
    std::cout << "result:" << std::bitset<64>(v) << std::endl;
  }
  return 0;
}
