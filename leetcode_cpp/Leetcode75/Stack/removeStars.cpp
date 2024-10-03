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
    static string removeStars(const string& s) {
        std::string ret;
        for (auto c : s) {
            if (c != '*') {
                ret.push_back(c);
            } else {
                ret.pop_back();
            }
        }
        return ret;
    }
};