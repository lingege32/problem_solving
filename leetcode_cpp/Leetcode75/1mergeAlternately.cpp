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
    static string mergeAlternately(string word1, string word2) {
        std::string ret;
        int w1 = word1.size();
        int w2 = word2.size();
        int s = std::min(w1, w2);
        ret.reserve(word1.size() + word2.size());
        for (int i = 0; i < s; ++i) {
            ret.push_back(word1[i]);
            ret.push_back(word2[i]);
        }
        for (; s < w1; ++s) {
            ret.push_back(word1[s]);
        }
        for (; s < w2; ++s) {
            ret.push_back(word2[s]);
        }
        return ret;
    }
};