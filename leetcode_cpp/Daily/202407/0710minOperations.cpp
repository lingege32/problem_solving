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
    static int minOperations(const vector<string>& logs) {
        int ans = 0;
        for (const auto& log : logs) {
            if (log[0] == '.') {
                if (log[1] == '.' && ans) {
                    ans--;
                }
            } else {
                ans++;
            }
        }
        return ans;
    }
};