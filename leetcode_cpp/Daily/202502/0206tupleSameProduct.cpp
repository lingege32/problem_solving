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
    static int tupleSameProduct(vector<int>& nums) {
        const int n = nums.size();
        std::vector<int> products(n * (n - 1) / 2);
        int idx = 0;
        for (int i = 0; i < n; ++i) {
            for (int j = i + 1; j < n; ++j) {
                products[idx++] = nums[i] * nums[j];
            }
        }
        std::ranges::sort(products);

        int last = products[0];
        int ret = 0;
        int group = 1;
        for (auto product : products | std::views::drop(1)) {
            if (product == last) {
                group++;
            } else {
                ret += 8 * (group * (group - 1) / 2);
                last = product;
                group = 1;
            }
        }
        ret += group * (group - 1) / 2;
        return ret;
    }
};