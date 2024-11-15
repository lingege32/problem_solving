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
    static int findLengthOfShortestSubarray(vector<int>& arr) {
        int n = arr.size();
        int l = 1;
        for (; l < n; ++l) {
            if (arr[l] < arr[l - 1]) {
                break;
            }
        }
        if (l == n) {
            return 0;
        }

        int r = n - 1;
        for (; r != 0; --r) {
            if (arr[r - 1] > arr[r]) {
                break;
            }
        }
        int max = std::max(n - r, l);
        auto it = std::next(arr.begin(), r);
        for (int i = 0; i < l && it != arr.end(); ++i) {
            auto v = arr[i];
            it = std::lower_bound(it, arr.end(), v);
            max = std::max(max, static_cast<int>(std::distance(it, arr.end())) + i + 1);
        }

        return n - max;
    }
};