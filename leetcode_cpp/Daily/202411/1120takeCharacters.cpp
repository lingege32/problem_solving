#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int takeCharacters(string s, int k) {
        int n = s.size();
        std::array<int, 3> cnt;
        cnt.fill(0);
        for (auto& c : s) {
            c -= 'a';
            cnt[c]++;
        }
        if (*std::ranges::min_element(cnt) < k) {
            return -1;
        }
        int ret = n;
        for (int i = 0, j = 0; i < n; ++i) {
            while (j < n && cnt[s[j]] > k) {
                cnt[s[j++]]--;
            }
            ret = std::min(ret, n - j + i);
            cnt[s[i]]++;
        }
        return ret;
    }
};
