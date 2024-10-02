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
    static bool isSubsequence(const string& s, const string& t) {
        if (s.empty()) {
            return true;
        }
        int n = s.size();
        int l = 0;
        for (auto c : t) {
            if (c == s[l]) {
                if (++l == n) {
                    return true;
                }
            }
        }
        return false;
    }
};