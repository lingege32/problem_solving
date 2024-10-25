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
    static int tribonacci(int n) {
        if (n < 3) {
            return n == 0 ? 0 : 1;
        }
        int v1 = 0, v2 = 1, v3 = 1;
        int cur;
        for (int i = 3; i <= n; ++i) {
            cur = v1 + v2 + v3;
            v1 = v2;
            v2 = v3;
            v3 = cur;
        }
        return v3;
    }
};