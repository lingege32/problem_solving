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
    static int minChanges(const string& s) {
        int ans = 0;
        size_t len = s.size();
        for (size_t i = 1; i < len; i += 2) {
            ans += (s[i] != s[i - 1]);
        }
        return ans;
    }
};