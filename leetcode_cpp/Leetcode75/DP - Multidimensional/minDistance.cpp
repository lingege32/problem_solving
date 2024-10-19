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
    static int minDistance(const string& word1, const string& word2) {
        int m = word1.size();
        int n = word2.size();
        std::vector<int> prev(n + 1, 0);
        std::vector<int> curr = prev;
        for (int i = 0; i <= n; ++i) {
            prev[i] = i;
        }
        for (int i = 1; i <= m; ++i) {
            curr[0] = i;
            for (int j = 1; j <= n; ++j) {
                if (word1[i - 1] == word2[j - 1]) {
                    curr[j] = prev[j - 1];
                } else {
                    curr[j] = std::min({prev[j - 1], prev[j], curr[j - 1]}) + 1;
                }
            }
            prev.swap(curr);
        }
        return prev.back();
    }
};