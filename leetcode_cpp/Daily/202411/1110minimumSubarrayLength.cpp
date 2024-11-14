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
    static int convert(const std::array<int, 32>& bits) {
        int kk = 0;
        for (int i = 0; i < 32; ++i) {
            kk |= (bits[i] > 0) << i;
        }
        return kk;
    }

    template <typename F>
    static void op(std::array<int, 32>& bits, int num, F&& fun) {
        int i = 0;
        while (num) {
            fun(bits[i], num & 1);
            num >>= 1;
            i++;
        }
    }

    static int minimumSubarrayLength(vector<int>& nums, int k) {
        if (k == 0) {
            return nums.empty() ? -1 : 1;
        }
        std::array<int, 32> bits;
        bits.fill(0);
        int left = 0;
        int right = 0;
        int n = nums.size();
        int ret = std::numeric_limits<int>::max();
        int val = 0;
        for (; right < n; ++right) {
            auto num = nums[right];
            val |= num;
            op(bits, nums[right], [](int& a, int b) { a += b; });
            while (val >= k) {
                ret = std::min(ret, right - left + 1);
                op(bits, nums[left], [](int& a, int b) { a -= b; });
                val = convert(bits);
                left++;
            }
        }
        return ret == std::numeric_limits<int>::max() ? -1 : ret;
    }
};

class OptSolution {
  public:
    static int minimumSubarrayLength(vector<int>& nums, int k) {
        if (k == 0) {
            return 1;
        }

        int n = nums.size();
        int l = 0;
        int curr_or = 0;
        std::array<int, 32> count;
        count.fill(0);
        int minLength = n + 1;
        for (int r = 0; r < n; r++) {
            curr_or |= nums[r];
            for (int i = 0, rnum = nums[r]; rnum > 0; i++) {
                count[i] += rnum & 1;
                rnum >>= 1;
            }

            while (curr_or >= k) {
                int len = r - l + 1;
                minLength = min(minLength, len);

                for (int i = 0, lnum = nums[l]; lnum > 0; i++) {
                    count[i] -= lnum & 1;
                    if (count[i] == 0) {
                        curr_or &= ~(1 << i);
                    }
                    lnum >>= 1;
                }
                l++;
            }
        }
        return minLength <= n ? minLength : -1;
    }
};