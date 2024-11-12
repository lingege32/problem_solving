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
    static string compressedString(const string& word) {
        string ans;
        char tmp_char = word[0];
        int now_cnt = 1;

        for (auto c : word | std::views::drop(1)) {
            if (c == tmp_char && now_cnt < 9) {
                now_cnt++;
            } else {
                ans += char(now_cnt) + '0';
                ans += tmp_char;
                tmp_char = c;
                now_cnt = 1;
            }
        }

        ans += char(now_cnt) + '0';
        ans += tmp_char;

        return ans;
    }
};