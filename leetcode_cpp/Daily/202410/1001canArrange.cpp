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
    static bool canArrange(vector<int>& arr, int k) {
        std::vector<int> mod(k, 0);
        for (auto a : arr) {
            auto i = a % k;
            mod[i < 0 ? i + k : i]++;
        }
        if ((mod[0] & 0x1) != 0) {
            return false;
        }
        int left = 1;
        int right = k - 1;
        for (; left < right;) {
            if (mod[left++] != mod[right--]) {
                return false;
            }
        }

        // no need to check left == right
        // because mod[0] is even
        // mod[1] + mod[k-1], mod[2] + mod[k-2],... are even
        // if arr.size() is even and mod[0] + ... + mod[left-1] + mod[left+1] + ...mod[k-1] is even
        // we can said that mod[left] is also even.

        // if (left == right) {
        //     if (mod[left] % 2 != 0) {
        //         return false;
        //     }
        // }
        return true;
    }
};