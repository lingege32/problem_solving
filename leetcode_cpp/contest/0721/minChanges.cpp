#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    int minChanges(int n, int k) {
        int ret = 0;
        while (n != 0) {
            if (n % 2 == 1 && k % 2 == 0) {
                ret += 1;
            }
            if (n % 2 == 0 && k % 2 == 1) {
                return -1;
            }
            n /= 2;
            k /= 2;
        }
        return k == 0 ? ret : -1;
    }
};