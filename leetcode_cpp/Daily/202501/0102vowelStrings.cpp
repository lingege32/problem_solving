#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

const inline std::vector<char> vowels = {'a', 'e', 'i', 'o', 'u'};

class Solution {
  public:
    static vector<int> vowelStrings(vector<string>& words, vector<vector<int>>& queries) {
        int len = words.size();
        vector<int> vowelCnt;
        vowelCnt.reserve(len + 1);
        vowelCnt.push_back(0);
        for (int i = 1; i < len + 1; ++i) {
            if (std::ranges::find(vowels, words[i - 1][0]) != vowels.end() &&
                std::ranges::find(vowels, words[i - 1].back()) != vowels.end()) {
                vowelCnt.push_back(vowelCnt[i - 1] + 1);
            } else {
                vowelCnt.push_back(vowelCnt[i - 1]);
            }
        }

        std::vector<int> ret;
        ret.reserve(queries.size());
        for (auto& pair : queries) {
            ret.push_back(vowelCnt[pair[1] + 1] - vowelCnt[pair[0]]);
        }

        return ret;
    }
};