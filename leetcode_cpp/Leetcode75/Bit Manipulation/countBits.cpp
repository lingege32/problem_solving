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
    static vector<int> countBits(int n) {
        std::vector<int> ret(n + 1, 0);
        for (int i = 1; i <= n; ++i) {
            ret[i] = ret[i / 2] + i % 2;
        }
        return ret;
    }
};