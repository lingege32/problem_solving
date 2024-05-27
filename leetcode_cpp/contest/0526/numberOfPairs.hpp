#include <vector>
using namespace std;

class Solution {
  public:
    int numberOfPairs(vector<int> &nums1, vector<int> &nums2, int k) {
        for (auto &n : nums1) {
            if (n % k == 0) {
                n /= k;
            } else {
                n = 0;
            }
        }
        return inner(nums1, nums2);
    }

    int inner(const vector<int> &n1, const vector<int> &n2) {
        int ans = 0;
        for (auto e1 : n1) {
            if (e1==0) {
                continue;
            }
            for (auto e2 : n2) {
                if (e1 % e2 == 0) {
                    ans++;
                }
            }
        }
        return ans;
    }
};