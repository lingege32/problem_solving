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
    static int minHeightShelves(vector<vector<int>>& books, int shelfWidth) {
        int n = books.size();
        std::vector<int> dp(n + 1, std::numeric_limits<int>::max());
        dp[0] = 0;
        for (int i = 1; i <= n; ++i) {
            auto total_width = 0;
            auto max_height = 0;
            for (int j = i; j > 0; --j) {
                auto& book = books[j - 1];
                auto width = book[0];
                auto height = book[1];
                max_height = std::max(max_height, height);
                total_width += width;
                if (total_width > shelfWidth) {
                    break;
                }
                dp[i] = std::min(dp[i], dp[j - 1] + max_height);
            }
        }
        return dp[n];
    }
};