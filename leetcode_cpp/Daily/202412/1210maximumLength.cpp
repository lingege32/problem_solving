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
    static int maximumLength(const string& s) {
        int n = s.size();
        std::array<int, 26> cnt{};
        cnt.fill(0);
        for (auto c : s) {
            cnt[c - 'a']++;
        }
        if (std::ranges::none_of(cnt, [](int x) { return x > 2; })) {
            return -1;
        }
        int left = 1, right = s.size() - 1;
        while (left < right) {
            int mid = (left + right) / 2;
            cnt.fill(0);
            bool flag = false;
            for (int i = mid; i <= n; ++i) {
                auto begin = i - mid;
                auto c = s[begin];
                if (std::all_of(s.begin() + begin, s.begin() + begin + mid, [c](char x) { return x == c; })) {
                    cnt[c - 'a']++;
                    if (cnt[c - 'a'] == 3) {
                        flag = true;
                        break;
                    }
                }
            }
            if (flag) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left == 1 ? 1 : left - 1;
    }
};