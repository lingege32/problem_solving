#include "header1.h"

int Solution::bitwiseComplement(int n) {
  int ans = 0;
  int mask = 1 << 30;
  while (mask > 0 && (mask & n) == 0) {
    mask >>= 1;
  }
  //   std::cout<<(void*)mask<<", "<<(void*)n<<std::endl;
  while (mask > 0) {
    ans <<= 1;
    if ((mask & n) == 0) {
      ans += 1;
    }
    mask >>= 1;
  }
  //   std::cout<<ans<<std::endl;
  return ans;
}
