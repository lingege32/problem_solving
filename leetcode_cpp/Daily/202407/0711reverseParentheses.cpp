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
    static string reverseParentheses(const string& s) {
        std::vector<std::string> stack;
        std::string cur;

        for (auto c : s) {
            if (c == '(') {
                stack.push_back(std::move(cur));
            } else if (c == ')') {
                std::ranges::reverse(cur);
                auto& b = stack.back();
                b += cur;
                std::swap(b, cur);
                stack.pop_back();
            } else {
                cur.push_back(c);
            }
        }

        return cur;
    }
};