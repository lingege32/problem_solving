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
    static int maxArea(vector<int>& height) {
        int l = 0;
        int r = height.size() - 1;
        int max = 0;
        int lh = height[l];
        int rh = height[r];
        while (l < r) {
            if (lh < rh) {
                max = std::max(max, (r - l) * lh);
                l++;
                lh = height[l];
            } else {
                max = std::max(max, (r - l) * rh);
                r--;
                rh = height[r];
            }
        }
        return max;
    }
};