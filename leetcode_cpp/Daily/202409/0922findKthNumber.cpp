#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int gap(long n, long a, long b) {
        int ret = 0;
        while (a <= n) {
            ret += (std::min(n + 1, b) - a);
            a *= 10;
            b *= 10;
        }
        return ret;
    }

    static int findKthNumber(int n, int k) {
        int start = 1;
        for (int i = 1; i < k;) {
            auto g = gap(n, start, start + 1);
            if (i + g <= k) {
                i += g;
                start++;
            } else {
                i++;
                start *= 10;
            }
        }
        return start;
    }
};