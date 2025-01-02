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
    static int maxScore(const string& s) {
        int score = std::count_if(s.begin(), s.end(), [](char c) { return c == '1'; });
        int ret = std::numeric_limits<int>::min();
        int len = s.length();
        for (int i = 0; i < len - 1; ++i) {
            if (s[i] == '0') {
                score++;
            } else {
                score--;
            }
            ret = std::max(ret, score);
        }
        return ret;
    }
};