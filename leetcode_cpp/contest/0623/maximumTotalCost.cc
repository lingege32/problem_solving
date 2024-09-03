#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    long long maximumTotalCost(vector<int> &nums) {
        size_t len = nums.size();
        if (len == 1) {
            return nums[0];
        }

        std::vector<long long> dp(len, 0);
        dp[0] = nums[0];
        if (nums[1] > 0) {
            dp[1] = dp[0] + nums[1];
        } else {
            dp[1] = dp[0] - nums[1];
        };

        for (size_t i = 2; i < len; ++i) {
            dp[i] = dp[i - 1] + nums[i];
            if (nums[i] < 0) {
                dp[i] = std::max(dp[i], dp[i - 2] + nums[i - 1] - nums[i]);
            }
        }

        return dp.back();
    }
};

/*
class Solution {
public:
    long long maximumTotalCost(vector<int>& nums) {
        int n = nums.size();

        long long ans = 0;
        vector<long long> dp(n, -1e15);
        dp[0] = nums[0];

        if(n == 1){
            return (long long)nums[0];
        }
        // I have created base case for first 2 element.
        if(nums[1] < 0){
            dp[1] = -1 * nums[1] + nums[0];
        }else{
            dp[1] = dp[0] + nums[1];
        }


        for(int i = 2 ; i < n ; i++){
            if(nums[i] < 0){
                dp[i] = dp[i - 2] + -1 * nums[i] + nums[i-1] ;
            }
            dp[i] = max(dp[i] , dp[i - 1] + nums[i]);
        }

        return dp[n - 1];

    }
};

*/