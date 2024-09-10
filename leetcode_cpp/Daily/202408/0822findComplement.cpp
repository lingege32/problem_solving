#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int findComplement(int num) {
        long n = num;
        long mask = 1;
        while (n) {
            mask <<= 1;
            n /= 2;
        }
        mask -= 1;
        return static_cast<long>(num) ^ mask;
    }
};