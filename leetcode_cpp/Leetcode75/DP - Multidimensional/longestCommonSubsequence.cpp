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
    static int longestCommonSubsequence(const string& text1, const string& text2) {
        int m = text1.size();
        int n = text2.size();
        std::vector<int> prev(n + 1, 0);
        std::vector<int> curr = prev;
        for (int i = 1; i <= m; ++i) {
            for (int j = 1; j <= n; ++j) {
                if (text1[i - 1] == text2[j - 1]) {
                    curr[j] = prev[j - 1] + 1;
                } else {
                    curr[j] = std::max(prev[j], curr[j - 1]);
                }
            }
            prev.swap(curr);
        }

        return prev.back();
    }
};