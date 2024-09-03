#include <bits/stdc++.h>
#include <limits>
using namespace std;
class Solution {
  public:
    size_t n;
    int m_max = std::numeric_limits<int>::max();
    void helper(vector<int> &ret, int score, const vector<int> &nums,
                vector<int> &vec, int mask) {
        if (score >= m_max) {
            return;
        }
        auto s = vec.size();
        if (s == n) {
            score += std::abs(vec[s - 1] - nums[vec[0]]);
            // std::cout<<"n: "<<n<<"score, "<<score<<", ";
            // for(auto v: vec) {
            //     std::cout<<v<<" ";
            // }
            // std::cout<<"\n";
            if (score < m_max) {
                m_max = score;
                ret = vec;
            }
        }
        for (size_t idx = 0; idx < n; ++idx) {

            auto new_mask = 1 << idx;
            if ((mask & new_mask) == 0) {
                vec.push_back(idx);
                helper(ret, score + std::abs(vec[s - 1] - nums[vec[s]]), nums,
                       vec, mask | new_mask);
                vec.pop_back();
            }
        }
    }
    vector<int> findPermutation(vector<int> &nums) {
        n = nums.size();
        vector<int> vec;
        vector<int> perm;
        for (auto idx = 0; idx < 1; ++idx) {
            vec.push_back(idx);
            helper(perm, 0, nums, vec, 1 << idx);
            vec.pop_back();
        }
        return perm;
    }
};