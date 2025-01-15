#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int minimizeXor(int num1, int num2) {
        int s = 0;
        while (num2 > 0) {
            s += (num2 & 1);
            num2 >>= 1;
        }
        int mask = 0x40000000;
        int ret = 0;
        while (mask > 0 && s > 0) {
            if (mask & num1) {
                ret |= mask;
                s--;
            }
            mask >>= 1;
        }
        mask = 1;
        while (s > 0) {
            if (!(mask & ret)) {
                ret |= mask;
                s--;
            }
            mask <<= 1;
        }
        return ret;
    }
};