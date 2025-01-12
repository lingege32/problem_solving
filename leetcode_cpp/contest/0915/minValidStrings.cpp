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
    static std::vector<int> prefix_of_target(std::string word, const std::string& target) {
        word += "#";
        word += target;
        int n = word.size();
        std::vector<int> pi(n, 0);
        for (int i = 1; i < n; ++i) {
            int j = pi[i - 1];
            while (j > 0 && word[i] != word[j]) {
                j = pi[j - 1];
            }
            if (word[i] == word[j]) {
                j++;
            }
            pi[i] = j;
        }

        return pi;
    }

    static int minValidStrings(vector<string>& words, const string& target) {
        // first step is to calculate the back N characters of index i of target.
        // can be a prefix.
        std::vector<int> back(target.size(), 0);
        for (auto& word : words) {
            auto pi = prefix_of_target(word, target);
            for (size_t i = 0; i < target.size(); ++i) {
                back[i] = std::max(back[i], pi[i + word.size() + 1]);
            }
        }

        if (std::ranges::any_of(back, [](int x) { return x == 0; })) {
            return -1;
        }

        // second step use dp to calculate
        std::vector<int> dp(target.size() + 1, 1e9);
        dp[0] = 0;
        for (size_t i = 0; i < target.size(); ++i) {
            dp[i + 1] = dp[i + 1 - back[i]] + 1;
        }
        return dp.back();
    }
};