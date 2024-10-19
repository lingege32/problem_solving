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
    static char findKthBit(int /*n*/, int k) {
        int top = 1;
        while (top < k) {
            top = 2 * top + 1;
        }
        bool isFlip = false;
        for (;;) {
            if (k == 1) {
                return isFlip ? '1' : '0';
            }
            auto mid = (top / 2) + 1;
            if (k == mid) {
                return isFlip ? '0' : '1';
            }
            if (k > mid) {
                k = mid - (k - mid);
                isFlip = !isFlip;
            }
            top = mid - 1;
        }

        std::unreachable();
    }
};