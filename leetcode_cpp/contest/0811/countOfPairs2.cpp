#include <bits/stdc++.h>
using namespace std;
#define MOD 1000000007
class Solution {
public:
    int countOfPairs(vector<int>& arr) {
        int n = arr.size();
        int mx = *max_element(arr.begin(),arr.end());
        vector<vector<int>>dp(n,vector<int>(mx+1,0));

        for(int i=0;i<=arr[0];i++){
            dp[0][i] = 1;
        }

        for(int i=1;i<n;i++){
            int cnt = 0;
            int k =0;
            for(int j=0;j<=arr[i];j++){
                if(k<=min(j,j-(arr[i]-arr[i-1]))){
                    cnt = (cnt + dp[i-1][k]) % MOD;
                    k++;
                }
                dp[i][j] = cnt;
            }
        }

        int ans = 0;
        for(int i=0;i<=mx;i++){
            ans = (ans + dp[n-1][i]) % MOD;
        }
        
        return ans;
    }
};