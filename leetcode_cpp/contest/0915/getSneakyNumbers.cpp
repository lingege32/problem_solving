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
    static vector<int> getSneakyNumbers(vector<int>& nums) {
        int n = nums.size();
        vector<int> ans, count(n);
        for (auto it : nums) {
            if (count[it]++ == 1) {
                ans.push_back(it);
            }
        }
        return ans;
    }
};