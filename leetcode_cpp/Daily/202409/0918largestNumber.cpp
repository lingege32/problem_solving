#include <bits/stdc++.h>

using namespace std;

class Solution {
  public:
    static string largestNumber(const vector<int>& nums) {
        std::vector<std::string> nums_str;
        nums_str.reserve(nums.size());
        for (auto n : nums) {
            nums_str.push_back(std::to_string(n));
        }
        std::ranges::sort(nums_str,
                          [](const std::string& lhs, const std::string& rhs) { return (rhs + lhs) < (lhs + rhs); });
        if (nums_str[0][0] == '0') {
            return "0";
        }
        std::string ret;
        for (const auto& s : nums_str) {
            ret += s;
        }
        return ret;
    }
};