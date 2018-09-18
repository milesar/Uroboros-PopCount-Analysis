#include <iostream>

int popcount_1_control(uint64_t x) {
  int c = 0;
  for (int i = 0; i < 64; i++) {
    if (x & 1)
      c++;
    x >>= 1;
    std::cout << "c = " << c << "\n";
    std::cout << "x = " << std::bitset<64>(x)  << "\n";
  }
  return c;
}

int popcount_kernighan(uint64_t n) {
    int count = 0;
    while (n) {
        count += n & 0x1u;
        n >>= 1;
        std::cout << "count = " << count << "\n";
        std::cout << "n = " << std::bitset<64>(n)  << "\n";
    }
    return count;
}

static const uint64_t m1  = 0x5555555555555555; //binary: 0101...
static const uint64_t m2  = 0x3333333333333333; //binary: 00110011..
static const uint64_t m4  = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
static const uint64_t h01 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...

int popcount64c(uint64_t x) {
    x -= (x >> 1) & m1;             //put count of each 2 bits into those 2 bits
    std::cout << "x = " << std::bitset<64>(x)  << "\n";
    x = (x & m2) + ((x >> 2) & m2); //put count of each 4 bits into those 4 bits
    std::cout << "x = " << std::bitset<64>(x)  << "\n";
    x = (x + (x >> 4)) & m4;        //put count of each 8 bits into those 8 bits
    std::cout << "x = " << std::bitset<64>(x)  << "\n";
    return (x * h01) >> 56;         //returns left 8 bits of x + (x<<8) + (x<<16) + (x<<24) + ...
}

int main() {
    uint64_t test = 34570;
    int result = popcount64c(test);
    std::cout << result << "\n";
    return 0;
}