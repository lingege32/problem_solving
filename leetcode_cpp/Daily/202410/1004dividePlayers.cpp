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
    static long long dividePlayers(vector<int>& skill) {
        int n = skill.size();
        int g = n / 2;
        int total = std::accumulate(skill.begin(), skill.end(), 0);
        if (total % g != 0) {
            return -1;
        }
        int each_g = total / g;
        std::map<int, int> m;
        long long sum = 0;
        for (auto i : skill) {
            long long remain = each_g - i;
            auto it = m.find(remain);
            if (it != m.end() && it->second != 0) {
                sum += i * remain;
                it->second--;
            } else {
                m[i]++;
            }
        }

        return std::ranges::all_of(m, [](const auto& p) { return p.second == 0; }) ? sum : -1;
    }
};
