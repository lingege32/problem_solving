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
    static int numberOfSubstrings(const string& s, int k) {
        std::array<int, 26> freq;
        int n = s.size();
        int cnt = 0;
        int ret = 0;
        int right = 0;
        for (int left = 0; left < n; ++left) {
            while (cnt == 0 && right < n) {
                if (++freq[s[right++] - 'a'] == k) {
                    cnt++;
                }
            }
            if (cnt != 0) {
                ret += (n - right + 1);
            } else if (right + 1 == n) {
                break;
            }
            if (freq[s[left] - 'a']-- == k) {
                cnt--;
            }
        }
        return ret;
    }
};