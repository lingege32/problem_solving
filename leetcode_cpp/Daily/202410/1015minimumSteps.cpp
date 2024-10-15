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
    static long long minimumSteps(const string& s) {
        long long ans = 0;
        for (int l = 0, r = s.length() - 1; l < r; l++) {
            while (l < r && s[l] == '1') {
                if (s[r] == '1') {
                    r--;
                } else {
                    ans += r-- - l;
                    break;
                }
            }
        }
        return ans;
    }
};