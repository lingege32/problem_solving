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
    static vector<string> stringSequence(const string& target) {
        int n = 0;
        for (auto c : target) {
            n += (c - 'a' + 1);
        }
        std::vector<std::string> ret(n);
        ret[0] = "a";
        for (int i = 1; i < n; ++i) {
            auto& word = ret[i];
            word = ret[i - 1];
            auto s = word.size();
            if (target[s - 1] == word[s - 1]) {
                word += 'a';
            } else {
                word.back()++;
            }
        }
        return ret;
    }
};

class OptSolution {
  public:
    static vector<string> stringSequence(string target) {
        int n = accumulate(target.begin(), target.end(), 0) - (('a' - 1) * target.length());
        vector<string> result;
        result.reserve(n);
        string curr;
        curr.reserve(target.length());
        for (char c : target) {
            curr.push_back('a' - 1);
            while (curr.back() < c) {
                curr.back()++;
                result.push_back(curr);
            }
        }
        return result;
    }
};