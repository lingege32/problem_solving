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
    static bool reportSpam(vector<string>& message, vector<string>& bannedWords) {
        std::unordered_set<std::string> banned{bannedWords.begin(), bannedWords.end()};
        int n = 0;
        for (const auto& word : message) {
            if (banned.contains(word) && n++ == 1) {
                return true;
            }
        }
        return false;
    }
};