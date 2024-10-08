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
    static int minSwaps(const string& s) {
        std::vector<char> stack;
        for (auto c : s) {
            if (c == '[') {
                stack.push_back(c);
            } else {
                if (!stack.empty() && stack.back() == '[') {
                    stack.pop_back();
                }
            }
        }

        return (stack.size() + 1) / 2;
    }
};