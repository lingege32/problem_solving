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
    static long long maxScore(vector<int>& nums1, vector<int>& nums2, int k) {
        size_t kk = k;
        int n = nums1.size();
        std::vector<std::pair<int, int>> v;
        v.reserve(n);
        for (int i = 0; i < n; ++i) {
            v.emplace_back(nums2[i], nums1[i]);
        }
        std::ranges::sort(v);
        std::priority_queue<int, vector<int>, std::greater<>> min_heap;
        long long ans = 0, cur = 0;
        for (int i = n - 1; i >= 0; --i) {
            min_heap.push(v[i].second);
            cur += v[i].second;
            if (min_heap.size() > kk) {
                cur -= min_heap.top();
                min_heap.pop();
            }
            if (min_heap.size() == kk) {
                ans = std::max(ans, cur * v[i].first);
            }
        }
        return ans;
    }
};