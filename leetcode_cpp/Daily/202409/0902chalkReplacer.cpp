#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    Solution() {
        std::ios_base::sync_with_stdio(false);
        std::cin.tie(nullptr);
        std::cout.tie(nullptr);
    }

    int chalkReplacer(vector<int>& chalk, int k) {
        std::vector<long long> chalk_sum(chalk.size(), 0);
        chalk_sum[0] = chalk[0];
        for (size_t i = 1; i < chalk_sum.size(); ++i) {
            chalk_sum[i] = chalk[i] + chalk_sum[i - 1];
        }
        k %= chalk_sum.back();
        return upper_bound(chalk_sum.begin(), chalk_sum.end(), k) - chalk_sum.begin();
    }
};

;