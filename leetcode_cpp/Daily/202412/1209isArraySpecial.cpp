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
    static vector<bool> isArraySpecial(vector<int>& nums, vector<vector<int>>& queries) {
        int n = nums.size();
        vector<int> maxR(n);
        int R = 0;
        for (int L = 0; L < n; L++) {
            // if R < L, set R = L
            R = max(R, L);
            // While R is not the last idx
            // And we can expand, then expand it.
            while (R < (n - 1) && nums[R] % 2 != nums[R + 1] % 2) {
                ++R;
            }

            maxR[L] = R;
        }
        vector<bool> res(queries.size());
        for (int i = 0, qn = queries.size(); i < qn; i++) {
            res[i] = queries[i][1] <= maxR[queries[i][0]];
        }
        return res;
    }
};