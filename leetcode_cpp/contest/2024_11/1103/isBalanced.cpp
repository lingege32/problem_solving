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
    static bool isBalanced(string num) {
        int n = num.length();
        int even = 0;
        int odd = 0;
        for (int i = 1; i < n; i += 2) {
            even += num[i - 1] - '0';
            odd += num[i] - '0';
        }
        if (n % 2 == 1) {
            even += num.back() - '0';
        }
        return even == odd;
    }
};