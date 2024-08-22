#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    bool doesAliceWin(string s) {
        int n = 0;
        for (auto c : s) {
            if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
                n += 1;
            }
        }
        return n != 0;
    }
};