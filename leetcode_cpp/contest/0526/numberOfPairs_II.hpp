#include <cmath>
#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
  public:
    long long numberOfPairs(vector<int> &nums1, vector<int> &nums2, int k) {
        long long ans = 0;
        unordered_map<int, long long> freq;
        for (auto n : nums2) {
            freq[n]++;
        }
        std::unordered_map<int, long long> freq1;
        for (auto n : nums1) {
            freq1[n]++;
        }

        for (auto [n, c] : freq1) {
            if (n % k != 0) {
                continue;
            }
            auto n1 = n / k;
            int sr = sqrt(n1);
            for (int i = 1; i <= sr; ++i) {
                if (n1 % i != 0) {
                    continue;
                }
                ans += c * freq[i];
                if (i != n1 / i) {
                    ans += c * freq[n1 / i];
                }
            }
        }
        return ans;
    }
};