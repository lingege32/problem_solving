#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    string largestPalindrome(int n, int k) {
        string ans(n, '9');
        if (k == 1 || k == 3 || k == 9) {
            return ans;
        }
        if (k == 2) {
            ans[0] = ans[n - 1] = '8';
        } else if (k == 4) {
            ans[0] = ans[n - 1] = '8';
            if (n > 1) {
                ans[1] = ans[n - 2] = '8';
            }
        } else if (k == 5) {
            ans[0] = ans[n - 1] = '5';
        } else if (k == 8) {
            ans[0] = ans[n - 1] = '8';
            if (n > 1) {
                ans[1] = ans[n - 2] = '8';
            }
            if (n > 2) {
                ans[2] = ans[n - 3] = '8';
            }
        } else if (k == 6) {
            if (n <= 2) {
                return string(n, '6');
            }
            if (n & 1) {
                ans[0] = ans[n >> 1] = ans[n - 1] = '8';
            } else {
                ans[0] = ans[n - 1] = '8';
                ans[(n >> 1) - 1] = ans[n >> 1] = '7';
            }
        } else if (k == 7) {
            if (n <= 2) {
                return string(n, '7');
            }
            for (int i = 9; i >= 0; i--) {
                if (!(n & 1)) {
                    ans[(n >> 1) - 1] = i + '0';
                }
                ans[n >> 1] = i + '0';
                if (isDivisibleBy7(ans)) {
                    return ans;
                }
            }
        }
        return ans;
    }

    bool isDivisibleBy7(const string& s) {
        int mod = 0;
        for (char digit : s) {
            mod = (mod * 10 + (digit - '0')) % 7;
        }
        return mod == 0;
    }
};