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
    static string makeFancyString(string s) {
        if (s.size() < 3) {
            return s;
        }
        size_t len = 2;
        for (size_t cur = 2; cur < s.size(); ++cur) {
            auto c = s[cur];
            if (c == s[len - 1] && c == s[len - 2]) {
                continue;
            }
            s[len] = c;
            len++;
        }

        s.resize(len);
        return s;
    }
};