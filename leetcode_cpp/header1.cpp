#include "header1.h"
#include <algorithm>
#include <memory.h>
#include <string_view>

int Solution::bitwiseComplement(int n) {
    if (n == 0) {
        return 1;
    }
    int ans = 0;
    int mask = 1 << 30;
    while (mask > 0 && (mask & n) == 0) {
        mask >>= 1;
    }
    //   std::cout<<(void*)mask<<", "<<(void*)n<<std::endl;
    while (mask > 0) {
        ans <<= 1;
        if ((mask & n) == 0) {
            ans += 1;
        }
        mask >>= 1;
    }
    //   std::cout<<ans<<std::endl;
    return ans;
}

vector<vector<string>> Solution::partition(string s) {
    struct Solution2 {
        bool dp[20][20];
        vector<string> curList;
        int n;

      public:
        vector<vector<string>> partition(string s) {
            vector<vector<string>> ans;
            n = s.length();

            memset(dp, false, sizeof dp);
            dfs(ans, s, 0);

            return ans;
        }

        void dfs(vector<vector<string>> &ans, string &s, int start) {
            if (start >= n) {
                ans.emplace_back(curList);
            }

            for (int i = start; i < n; i++) {
                if (s[start] == s[i] &&
                    (i - start <= 2 || dp[start + 1][i - 1])) {
                    dp[start][i] = true;
                    curList.emplace_back(s.substr(start, i - start + 1));
                    dfs(ans, s, i + 1);
                    curList.pop_back();
                }
            }
        }
    };
    Solution2 s2;
    return s2.partition(std::move(s));
}