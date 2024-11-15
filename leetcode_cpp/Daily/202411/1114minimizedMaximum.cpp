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
    static int minimizedMaximum(int n, vector<int>& quantities) {
        int right = *std::ranges::max_element(quantities);
        int left = 1;

        while (left < right) {
            int mid = left + ((right - left) / 2);
            int cnt = 0;
            for (auto q : quantities) {
                cnt += (q + mid - 1) / mid;
            }
            if (cnt > n) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left;
    }
};