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
    static int passThePillow(int n, int time) {
        time %= ((n - 1) * 2);
        if (time < n - 1) {
            return 1 + time;
        }
        time -= (n - 1);
        return n - time;
    }
};