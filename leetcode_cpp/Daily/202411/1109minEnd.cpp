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
    static long long minEnd(int n, int x) {
        n -= 1;
        long long ans = x;
        long long mask = 1;
        while (n) {
            while (ans & mask) {
                mask <<= 1;
            }
            auto bit = n % 2;
            n >>= 1;
            if (bit) {
                ans |= mask;
            }
            mask <<= 1;
        }
        return ans;
    }
};