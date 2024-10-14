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
    static long long maxKelements(vector<int>& nums, int k) {
        std::priority_queue<int> max_heap{nums.begin(), nums.end()};
        long long ret = 0;
        for (auto i = 0; i < k; ++i) {
            long long t = max_heap.top();
            max_heap.pop();
            ret += t;
            max_heap.emplace((t + 2) / 3);
        }
        return ret;
    }
};