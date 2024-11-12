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
    static bool rotateString(const string& s, const string& goal) {
        auto len = s.size();
        if (len != goal.size()) {
            return false;
        }

        auto c = s[0];
        auto pos = goal.find(c);
        for (; pos != std::string::npos; pos = goal.find(c, pos + 1)) {
            auto right = len - pos;
            if (goal.substr(pos) == s.substr(0, right) && goal.substr(0, pos) == s.substr(right)) {
                return true;
            }
        }
        return false;
    }
};