#include <bits/stdc++.h>

using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

std::array<int, 26> trans(const std::string& s) {
    std::array<int, 26> ret;
    ret.fill(0);
    for (auto c : s) {
        ret[c - 'a']++;
    }
    return ret;
}

class Solution {
  public:
    static vector<string> wordSubsets(vector<string>& words1, vector<string>& words2) {
        std::array<int, 26> patterns;
        patterns.fill(0);
        for (auto& word : words2) {
            auto r = trans(word);
            for (int i = 0; i < 26; i++) {
                patterns[i] = std::max(patterns[i], r[i]);
            }
        }

        auto it = std::remove_if(words1.begin(), words1.end(), [&](const std::string& word) {
            auto r = trans(word);
            for (int i = 0; i < 26; ++i) {
                if (r[i] > patterns[i]) {
                    return true;
                }
            }
            return false;
        });
        words1.erase(it, words1.end());
        return std::move(words1);
    }
};