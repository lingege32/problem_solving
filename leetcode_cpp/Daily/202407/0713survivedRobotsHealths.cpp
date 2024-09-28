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
    static vector<int> survivedRobotsHealths(vector<int>& positions, vector<int>& healths, string directions) {
        int n = positions.size();
        std::vector<int> robots;
        robots.reserve(n);
        for (int i = 0; i < n; ++i) {
            robots.push_back(i);
        }
        std::ranges::sort(robots, [&](int lhs, int rhs) { return positions[lhs] < positions[rhs]; });

        std::vector<int> stack;
        std::vector<int> ans;
        stack.reserve(n);
        ans.reserve(n);
        for (auto robot : robots) {
            if (directions[robot] == 'R') {
                stack.push_back(robot);
            } else {
                auto& h = healths[robot];
                while (!stack.empty() && h != 0) {
                    auto top = stack.back();
                    auto& ht = healths[top];
                    if (ht == h) {
                        h = 0;
                        stack.pop_back();
                    } else if (ht > h) {
                        h = 0;
                        ht--;
                    } else {
                        h--;
                        stack.pop_back();
                    }
                }
                if (h > 0) {
                    ans.push_back(robot);
                }
            }
        }
        ans.insert(ans.end(), stack.begin(), stack.end());
        std::ranges::sort(ans);
        for (auto& a : ans) {
            a = healths[a];
        }
        return ans;
    }
};