#include <bits/stdc++.h>

#include <algorithm>
using namespace std;

class Solution {
  public:
    static int rangeSum(vector<int>& nums, int /*n*/, int left, int right) {
        left--;
        right--;
        for (size_t i = 1; i < nums.size(); ++i) {
            nums[i] += nums[i - 1];
        }
        std::vector<int> v = nums;

        v.reserve((nums.size() * (nums.size() + 1)) / 2);
        for (size_t i = 1; i < nums.size(); ++i) {
            for (size_t j = 0; j < i; ++j) {
                v.push_back(nums[i] - nums[j]);
            }
        }
        std::nth_element(v.begin(), v.begin() + right, v.end());
        std::nth_element(v.begin(), v.begin() + left, v.begin() + right);
        long long ans = 0;
        for (auto be = std::next(v.begin(), left); be != std::next(v.begin(), right + 1); ++be) {
            ans += *be;
        }
        return ans % 1000000007;
    }
};

class OptimizeSolution {
  public:
    static constexpr int MODULUS = 1000000007;
    long long maxSubarraySum = 0;
    long long minSubarraySum = 0;

    static std::pair<int, long long> countAndSumSubarrays(const std::vector<int>& array, long long threshold) {
        int count = 0;
        long long totalSum = 0, currentWindowSum = 0, runningSum = 0;
        int size = array.size();

        for (int start = 0, end = 0; end < size; ++end) {
            runningSum += static_cast<long long>(array[end]) * (end - start + 1);
            currentWindowSum += array[end];
            while (currentWindowSum > threshold) {
                runningSum -= currentWindowSum;
                currentWindowSum -= array[start++];
            }
            count += end - start + 1;
            totalSum += runningSum;
        }
        return {count, totalSum};
    }

    [[nodiscard]] long long calculateSumOfFirstKSubarrays(const std::vector<int>& array, int k) const {
        long long low = minSubarraySum, high = maxSubarraySum;
        while (low < high) {
            long long mid = low + ((high - low) / 2);
            if (countAndSumSubarrays(array, mid).first < k) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        auto [count, sum] = countAndSumSubarrays(array, low);
        return sum - (low * (count - k));
    }

    int rangeSum(vector<int>& nums, int /*n*/, int left, int right) {
        minSubarraySum = *std::ranges::min_element(nums);
        maxSubarraySum = std::accumulate(nums.begin(), nums.end(), 0LL);

        long long result = (calculateSumOfFirstKSubarrays(nums, right) % MODULUS -
                            calculateSumOfFirstKSubarrays(nums, left - 1) % MODULUS + MODULUS) %
                           MODULUS;
        return static_cast<int>(result);
    }
};