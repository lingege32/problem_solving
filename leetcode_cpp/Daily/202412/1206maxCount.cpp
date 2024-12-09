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
    static int maxCount(vector<int>& banned, int n, int maxSum) {
        std::vector<int> arr(n, 1);
        for (auto i : banned) {
            if (i <= n) {
                arr[i - 1] = 0;
            }
        }
        int ret = 0;
        for (int i = 0; i < n; ++i) {
            if (arr[i] == 1) {
                maxSum -= (i + 1);
                if (maxSum < 0) {
                    break;
                }
                ret++;
            }
        }
        return ret;
    }
};