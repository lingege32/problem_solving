#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArr {
    std::vector<int> data;
    int m;

    explicit TwoDArr(int n):
      data(n * n, 0),
      m{n} {}

    int* operator[](int idx) { return &data[idx * m]; }
};

class Solution {
  public:
    static int lenLongestFibSubseq(vector<int>& arr) {
        TwoDArr dp(arr.size());
        int ret = 0;
        for (size_t i = 2; i < arr.size(); ++i) {
            size_t left = 0;
            size_t right = i - 1;
            int cur = arr[i];
            while (left < right) {
                int sum = arr[left] + arr[right];
                if (sum > cur) {
                    right--;
                } else if (sum < cur) {
                    left++;
                } else {
                    if (dp[left][right] == 0) {
                        dp[right][i] = 3;
                    } else {
                        dp[right][i] = dp[left][right] + 1;
                    }
                    ret = std::max(ret, dp[right][i]);
                    left++;
                    right--;
                }
            }
        }
        return ret;
    }
};
