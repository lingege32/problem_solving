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
    static vector<string> sortPeople(vector<string>& names, vector<int>& heights) {
        int n = names.size();
        std::vector<int> indices(n);
        for (int i = 0; i < n; ++i) {
            indices[i] = i;
        }
        std::ranges::sort(indices, [&heights](int lhs, int rhs) { return heights[lhs] > heights[rhs]; });
        std::vector<std::string> ret;
        ret.reserve(n);
        for (auto idx : indices) {
            ret.emplace_back(std::move(names[idx]));
        }

        return ret;
    }
};