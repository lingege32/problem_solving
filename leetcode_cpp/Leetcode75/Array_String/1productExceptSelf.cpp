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
    static vector<int> productExceptSelf(vector<int>& nums) {
        int n = 1;
        int zero_cnt = 0;
        for (auto num : nums) {
            if (num) {
                n *= num;
            } else {
                zero_cnt++;
            }
        }

        if (zero_cnt == 0) {
            for (auto& num : nums) {
                num = n / num;
            }
        } else if (zero_cnt == 1) {
            for (auto& num : nums) {
                if (num) {
                    num = 0;
                } else {
                    num = n;
                }
            }
        } else {
            for (auto& num : nums) {
                num = 0;
            }
        }
        return std::move(nums);
    }
};

class Otherolution {
public:
    static vector<int> productExceptSelf(vector<int>& nums) {
        const int n = nums.size();
        vector<int> result(n);
        
        int* res = result.data();
        const int* num = nums.data();
        int left_prod = 1;
        for (int i = 0; i < n; ++i) {
            res[i] = left_prod;
            left_prod *= num[i];
        }
        
        int right_prod = 1;
        for (int i = n - 1; i >= 0; --i) {
            res[i] *= right_prod;
            right_prod *= num[i];
        }
        
        return result;
    }
};
