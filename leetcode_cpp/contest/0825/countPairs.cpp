#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int solve(int x,int y){
        int pos = 0;
        int x1 = -1,y1 = -1,x2 = -1, y2 = -1;
        while(x>0 || y>0){
            if(x%10 != y%10){
                if(pos>=2) return 0;
                pos++;
                if(x1==-1){
                    x1=x%10;
                    y1=y%10;
                }else{
                    x2=x%10;
                    y2=y%10;
                }
            } 
            x/=10;
            y/=10;
        }
        if(x1==y2 && x2==y1) return 1;
        return 0;
    }
    int countPairs(vector<int>& nums) {
        int n = nums.size();
        int ans = 0;
        for(int i = 0; i < n;i++){
            for(int j = i+1; j < n;j++){
                int res = solve(nums[i],nums[j]);
                // if(res==1){
                //     cout<<nums[i]<<" "<<nums[j]<<endl;
                // }
                ans += res;
            }
        }
        return ans;
    }
};