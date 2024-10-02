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
    static int compress(vector<char>& chars) {
        size_t cur = 0;
        auto prev = chars[0];
        int count = 1;
        for (size_t i = 1; i < chars.size(); ++i) {
            auto c = chars[i];
            if (c == prev) {
                count++;
            } else {
                chars[cur] = prev;
                cur++;
                auto be = cur;
                if (count != 1) {
                    while (count > 0) {
                        chars[cur++] = '0' + count % 10;
                        count /= 10;
                    }
                }
                std::reverse(chars.begin() + be, chars.begin() + cur);
                prev = c;
                count = 1;
            }
        }
        chars[cur] = prev;
        cur++;
        auto be = cur;
        if (count != 1) {
            while (count > 0) {
                chars[cur++] = '0' + count % 10;
                count /= 10;
            }
        }
        std::reverse(chars.begin() + be, chars.begin() + cur);
        chars.resize(cur);

        return cur;
    }
};