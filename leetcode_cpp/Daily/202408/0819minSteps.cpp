#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int minSteps(int n) {
        int ans = 0;
        int d = 2;
        // follow the pattern, copy one and paste d-1 times
        while (n > 1) {
            // if n can be divied by d, it's means that we can copy n/2 and paste n/2, so we add d to ans;
            // until d cannot divide the n, then incremental it.
            while (n % d == 0) {
                ans += d;
                n /= d;
            }
            d++;
        }
        return ans;
    }
};