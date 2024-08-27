#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    bool canAliceWin(vector<int> &nums) {
        int s = 0;
        int d = 0;
        for (auto n : nums) {
            if (n < 10) {
                s += n;
            } else {
                d += n;
            }
        }
        return s != d;
    }
};