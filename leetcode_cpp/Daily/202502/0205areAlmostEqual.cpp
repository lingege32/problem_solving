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
    static bool areAlmostEqual(string s1, string s2) {
        if (s1.size() != s2.size()) {
            return false;
        }
        int f = -1, s = -1;
        for (size_t i = 0; i < s1.size(); ++i) {
            if (s1[i] != s2[i]) {
                if (f != -1) {
                    return false;
                }
                f = i;
                std::swap(f, s);
            }
        }
        return (f == -1 && s == -1) || (f != -1 && s != -1 && s1[f] == s2[s] && s1[s] == s2[f]);
    }
};