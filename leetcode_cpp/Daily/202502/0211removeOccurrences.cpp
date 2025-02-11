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
    static string removeOccurrences(string s, const string& part) {
        const size_t n = part.size();
        size_t len = 0;
        for (auto c : s) {
            s[len++] = c;
            if (len >= n && c == part.back() && s.substr(len - n, n) == part) {
                len -= n;
            }
        }
        s.resize(len);
        return s;
    }
};