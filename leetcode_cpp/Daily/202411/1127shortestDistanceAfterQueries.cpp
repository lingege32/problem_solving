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
    static vector<int> shortestDistanceAfterQueries(int n, vector<vector<int>>& queries) {
        std::vector<int> ret(queries.size(), 0);
        std::vector<int> arr(n, 0);
        std::vector<std::vector<int>> in(n);
        for (int step = n - 1; auto& v : arr) {
            v = step--;
        }
        for (int i = 1; i < n; ++i) {
            in[i].push_back(i - 1);
        }

        std::function<void(int)> visited = [&](int index) {
            for (auto prev : in[index]) {
                auto& p = arr[prev];
                auto cur = arr[index] + 1;
                if (p > cur) {
                    p = cur;
                    visited(prev);
                }
            }
        };
        int i = 0;
        for (const auto& query : queries) {
            auto n1 = query[0];
            auto n2 = query[1];
            in[n2].push_back(n1);
            visited(n2);
            ret[i++] = arr[0];
        }

        return ret;
    }
};
