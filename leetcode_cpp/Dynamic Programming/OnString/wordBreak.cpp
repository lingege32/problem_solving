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
    static bool wordBreak(const string& s, vector<string>& wordDict) {
        int len = s.length();
        std::vector<bool> dp(len + 1, false);
        std::unordered_set<std::string> dict(wordDict.begin(), wordDict.end());
        int max_length = 0;
        for (auto& ss : dict) {
            max_length = std::max(max_length, static_cast<int>(ss.length()));
        }

        dp[0] = true;
        for (int i = 1; i < len + 1; ++i) {
            for (int j = 1; j <= max_length && i - j >= 0 && !dp[i]; ++j) {
                dp[i] = dp[i - j] && dict.contains(s.substr(i - j, j));
            }
        }
        return dp.back();
    }
};