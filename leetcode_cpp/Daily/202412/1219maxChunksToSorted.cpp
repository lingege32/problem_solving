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
    static int maxChunksToSorted(const vector<int>& arr) {
        int ret = 0;
        auto it = arr.begin();
        for (; it != arr.end();) {
            auto pivotIt = std::min_element(it, arr.end()) + 1;
            auto max = *std::max_element(it, pivotIt);
            auto tgt = std::find_if(pivotIt, arr.end(), [&max](int i) { return i < max; });
            while (tgt != arr.end()) {
                pivotIt = tgt + 1;
                max = *std::max_element(it, pivotIt);
                tgt = std::find_if(pivotIt, arr.end(), [&max](int i) { return i < max; });
            }
            ret++;
            it = pivotIt;
        }

        return ret;
    }
};