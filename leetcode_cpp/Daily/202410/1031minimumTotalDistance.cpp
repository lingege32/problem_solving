#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static long long minimumTotalDistance(vector<int>& robot, vector<vector<int>>& factory) {
        // Sort both robots and factories for optimal assignment
        std::ranges::sort(robot);
        std::ranges::sort(factory);
        int m = robot.size();
        int n = factory.size();
        using LL = long long;
        // store the min_distance with robots[i..] and factories[j..]
        vector<vector<LL>> dp(m, vector<LL>(n + 1, std::numeric_limits<LL>::max()));

        for (int i = n - 1; i >= 0; --i) {
            // qq stores difference between if robot[j] is store in facotories[i] and factories[i+1]
            std::deque<std::pair<int, LL>> qq;
            qq.emplace_back(m, 0);
            LL prefix = 0;
            for (int j = m - 1; j >= 0; --j) {
                prefix += std::abs(robot[j] - factory[i][0]);
                // pop front if factory[i] is full after considering the robot[j]
                while (!qq.empty() && qq.front().first > j + factory[i][1]) {
                    qq.pop_front();
                }

                // maintain the increasing order of the qq[].second;
                auto diff = dp[j][i + 1] - prefix;
                while (!qq.empty() && qq.back().second >= diff) {
                    qq.pop_back();
                }

                qq.emplace_back(j, diff);
                dp[j][i] = prefix + qq.front().second;
            }
        }
        return dp[0][0];
    }
};