#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static bool lemonadeChange(vector<int>& bills) {
        int five = 0;
        int ten = 0;
        for (auto b : bills) {
            if (b == 5) {
                five++;
            } else if (b == 10) {
                if (five == 0) {
                    return false;
                }
                five--;
                ten++;
            } else {
                int need_five = 1;
                if (ten > 0) {
                    ten--;
                } else {
                    need_five = 3;
                }
                if (five >= need_five) {
                    five -= need_five;
                } else {
                    return false;
                }
            }
        }
        return true;
    }
};