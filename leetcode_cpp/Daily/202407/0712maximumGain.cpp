#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int maximumGain(string s, int x, int y) {
        if (x < y) {
            swap(x, y);
            std::ranges::reverse(s);
        }
        int ans = 0;
        int count_a = 0, count_b = 0;
        for (char& c : s) {
            if (c == 'b' && count_a) {
                count_a--;
                ans += x;
            } else {
                if (c == 'a') {
                    count_a++;
                } else if (c == 'b') {
                    count_b++;
                } else {
                    ans += min(count_b, count_a) * y;
                    count_a = 0, count_b = 0;
                }
            }
        }
        ans += min(count_b, count_a) * y;
        return ans;
    }
};
