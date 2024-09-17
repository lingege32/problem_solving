#include <bits/stdc++.h>
using namespace std;

class KthLargest {
  public:
    priority_queue<int, vector<int>, greater<>> pq;
    size_t K = 0;

    KthLargest(int k, vector<int>& nums):
      K{static_cast<size_t>(k)} {
        if (nums.size() > K) {
            sort(nums.begin(), nums.end(), greater<>());
        }
        auto n = nums.size();
        auto size = min(K, n);
        for (size_t i = 0; i < size; i++) {
            pq.push(nums[i]);
        }
        K = k;
    }

    int add(int val) {
        if (pq.empty()) {
            pq.push(val);
            return val;
        }
        if (pq.size() < K) {
            pq.push(val);
        } else {
            if (val > pq.top()) {
                pq.pop();
                pq.push(val);
            }
        }
        return pq.top();
    }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */