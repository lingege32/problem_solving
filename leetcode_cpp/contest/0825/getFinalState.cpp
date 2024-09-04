#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    vector<int> getFinalState(vector<int>& nums, int k, int multiplier) {
        std::map<int, set<size_t>> numbers;
        for (size_t idx = 0; idx < nums.size(); ++idx) {
            numbers[nums[idx]].insert(idx);
        }

        for (;;) {
            auto extracted = numbers.extract(numbers.begin());
            int len = extracted.mapped().size();
            if (k >= len) {
                // std::cout << "handle " << extracted.key() << std::endl;
                auto next = extracted.key() * multiplier;
                auto& next_set = numbers[next];
                for (auto idx : extracted.mapped()) {
                    nums[idx] *= multiplier;
                    next_set.insert(idx);
                }
                k -= len;
                // std::cout << k << std::endl;
            } else {
                for (auto idx : extracted.mapped()) {
                    if (k-- == 0) {
                        break;
                    }
                    nums[idx] *= multiplier;
                }
                break;
            }
        }

        return std::move(nums);
    }
};