#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    bool isPrime(int i) {
        for (int k = 2;; k++) {
            if (i % k == 0) {
                return i==2;
            }
            if (k * k > i) {
                break;
            }
        }
        return true;
    }
    int nonSpecialCount(int l, int r) {
        int c = 0;
        for (int i = 2;; i++) {
            auto ii = i * i;
            if (ii < l) {
                continue;
            }
            if (ii > r) {
                break;
            }
            if (isPrime(i)) {
                std::cout<<i<<std::endl;
                c++;
            }
        }
        return r - l + 1 - c;
    }
};