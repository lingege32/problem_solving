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
    static int fib(int n) {
        if (n < 2) {
            return n;
        }
        int prev1 = 0;
        int prev2 = 1;
        int cur;
        for (int i = 2; i <= n; ++i) {
            cur = prev1 + prev2;
            prev1 = prev2;
            prev2 = cur;
        }
        return prev2;
    }
};