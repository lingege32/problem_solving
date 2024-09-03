#include <algorithm>
#include <unordered_map>
#include <vector>
using namespace std;
class Solution {
  public:
    long long maximumTotalDamage(vector<int> &power) {
        std::unordered_map<long, long> power_count;
        for (auto p : power) {
            power_count[p]++;
        }
        if (power_count.size() == 1) {
            auto one = *power_count.begin();
            return one.first * one.second;
        }
        vector<pair<long, long>> new_power;
        new_power.reserve(power_count.size());
        for (const auto &p : power_count) {
            new_power.emplace_back(p);
        }

        std::sort(new_power.begin(), new_power.end());
        std::vector<long long> dp;
        dp.resize(new_power.size(), 0);
        dp.back() = new_power.back().first * new_power.back().second;
        for (int idx = dp.size() - 2; idx >= 0; --idx) {
            auto pow = new_power[idx];
            auto p = pow.first * pow.second;
            auto find = std::lower_bound(
                new_power.begin() + idx + 1, new_power.end(), pow.first + 3,
                [](const auto &lhs, long rhs) { return lhs.first < rhs; });
            auto v = dp[idx + 1];
            if (find == new_power.end()) {
                dp[idx] = (p > v ? p : v);
            } else {
                auto dis = std::distance(new_power.begin(), find);
                auto new_val = dp[dis] + p;
                dp[idx] = (new_val > v ? new_val : v);
            }
        }
        return dp.front();
    }
};