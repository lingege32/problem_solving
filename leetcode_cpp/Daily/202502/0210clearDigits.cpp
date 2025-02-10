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
    static string clearDigits(string s) {
        int size = 0;
        for (auto c : s) {
            if (c >= '0' && c <= '9') {
                size--;
            } else {
                s[size++] = c;
            }
        }
        s.resize(size);
        return s;
    }
};