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
    static string decodeString(const string& s) {
        std::vector<std::pair<int, std::string>> stack;
        std::string cur;
        int num = 0;
        for (auto c : s) {
            if (c >= '0' && c <= '9') {
                num *= 10;
                num += (c - '0');
            } else if (c >= 'a' && c <= 'z') {
                if (num > 0) {
                    for (int i = 0; i < num; ++i) {
                        cur.push_back(c);
                    }
                } else {
                    cur.push_back(c);
                }
                num = 0;
            } else if (c == '[') {
                stack.emplace_back(num, std::move(cur));
                num = 0;
            } else if (c == ']') {
                auto [n, prev] = stack.back();
                stack.pop_back();
                for (int i = 0; i < std::max(1, n); ++i) {
                    prev += cur;
                }
                cur = std::move(prev);
            }
        }
        return cur;
    }
};