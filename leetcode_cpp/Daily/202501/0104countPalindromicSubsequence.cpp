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
    static int countPalindromicSubsequence(const string& s) {
        int len = s.size();
        std::array<std::vector<int>, 26> pos;
        for (int i = 0; i < len; ++i) {
            pos[s[i] - 'a'].push_back(i);
        }
        int ret = 0;
        for (int o = 0; o < 26; ++o) {
            auto& p = pos[o];
            if (p.empty()) {
                continue;
            }
            auto next = p[0] + 1;
            for (int second = 0; second < 26; ++second) {
                auto& q = pos[second];
                if (q.empty()) {
                    continue;
                }
                auto sec_it = std::ranges::lower_bound(q, next);
                if (sec_it == q.end()) {
                    continue;
                }
                auto sec_pos = *sec_it;
                if (std::ranges::lower_bound(p, sec_pos + 1) != p.end()) {
                    ret++;
                }
            }
        }
        return ret;
    }
};