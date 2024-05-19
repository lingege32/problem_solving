#include <algorithm>
#include <iterator>
#include <vector>
#include <iostream>
using namespace std;
class Solution {
public:
    vector<bool> isArraySpecial(vector<int>& nums, vector<vector<int>>& queries) {
        std::vector<int> invalid_pairs;
        bool tag = 0==(nums[0]%2);
        for(size_t idx=1; idx<nums.size(); ++idx) {
            if ((tag && nums[idx]%2==0) || (!tag && nums[idx]%2==1)) {
                invalid_pairs.emplace_back(idx-1);
                tag = 0==(nums[idx]%2);
            } else {
                tag = !tag;
            }
        }
        std::vector<bool> ans;
        ans.reserve(queries.size());
        std::transform(queries.begin(), queries.end(), std::back_inserter(ans), [&](const std::vector<int>& query) {
            return this->isValid(invalid_pairs, query);
        });
        return ans;
    }

    bool isValid(const std::vector<int>& invalid, const std::vector<int>& query) {
        auto left = query[0];
        auto right = query[1];
        auto lower = std::lower_bound(invalid.begin(), invalid.end(), left);
        if (lower == invalid.end()) {
            return true;
        }
        auto upper = std::lower_bound(invalid.begin(), invalid.end(), right);
        return lower==upper;
    }
};