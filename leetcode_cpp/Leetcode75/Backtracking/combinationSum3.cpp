#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
    size_t mk = 0;

  public:
    vector<vector<int>> combinationSum3(int k, int n) {
        std::vector<vector<int>> ret;
        std::vector<int> path;
        mk = k;
        path.reserve(k);
        inner(ret, path, 1, n);
        return ret;
    }

    void inner(std::vector<std::vector<int>>& ans, std::vector<int>& path, int start, int target) {
        if (path.size() == mk) {
            if (target == 0) {
                ans.push_back(path);
            }
        }

        for (; start < 10; ++start) {
            if (target < start) {
                break;
            }
            path.push_back(start);
            inner(ans, path, start + 1, target - start);
            path.pop_back();
        }
    }
};