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
    static int minAddToMakeValid(const string& s) {
        int left = 0;
        int res = 0;
        for (auto c : s) {
            if (c == ')') {
                if (left != 0) {
                    left--;
                } else {
                    res++;
                }
            } else {
                left++;
            }
        }
        return left + res;
    }
};