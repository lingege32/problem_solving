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
    static int findPeakElement(vector<int>& v) {
        int l = 0;
        int h = v.size() - 1;
        while (l < h) {
            int mid = l + ((h - l) / 2);
            if (v[mid] > v[mid + 1]) {
                h = mid;
            } else {
                l = mid + 1;
            }
        }
        return l;
    }
};