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
    static int minFlips(int a, int b, int c) {
        int ans = 0;
        while (c != 0 || a != 0 || b != 0) {
            int bit_a = a & 1;
            int bit_b = b & 1;
            int bit_c = c & 1;
            a >>= 1;
            b >>= 1;
            c >>= 1;
            if (bit_c == 0) {
                ans += (bit_a + bit_b);
            } else {
                ans += (bit_a == 0 && bit_b == 0);
            }
        }
        return ans;
    }
};