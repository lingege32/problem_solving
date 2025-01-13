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
    static int minimumLength(const string& s) {
        std::array<int, 26> count;
        count.fill(0);
        for (auto c : s) {
            count[c - 'a']++;
        }
        int ret = 0;
        for (auto v : count) {
            ret += (1 + (v - 1) % 2);
        }
        return ret;
    }
};