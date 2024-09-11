#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int minBitFlips(int start, int goal) {
        auto mask = start ^ goal;
        // std::cout << mask << std::endl;
        int n = 0;
        while (mask) {
            n += (mask & 1);
            mask >>= 1;
        }
        return n;
    }
};