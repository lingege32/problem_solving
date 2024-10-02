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
    static vector<int> arrayRankTransform(vector<int>& arr) {
        if (arr.empty()) {
            return {};
        }
        int n = arr.size();
        std::vector<int> indices(n, 0);
        for (int i = 1; i < n; ++i) {
            indices[i] = i;
        }

        std::ranges::sort(indices, [&](int a, int b) { return arr[a] < arr[b]; });
        int prev = arr[indices[0]];
        int rank = 1;
        for (auto idx : indices) {
            auto& v = arr[idx];
            if (v != prev) {
                rank++;
            }
            prev = v;
            v = rank;
        }
        return std::move(arr);
    }
};