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
    static int longestOnes(vector<int>& nums, int k) {
        int ans = 0;
        int zero = 0;
        int l = 0;
        for(int r = 0; r < nums.size(); ++r) {
            if(nums[r] == 0) {
                zero++;
            }
            if(zero > k) {
                if(nums[l] == 0) {
                    zero--;
                }
                l++;
            } else {
                ans = max(ans, r - l + 1);
            }  
        }
        return ans;
    }
};