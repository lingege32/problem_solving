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
    static long long totalCost(vector<int>& costs, int k, int candidates) {
        size_t can = candidates;
        int i = 0;
        int n = costs.size();
        int j = n - 1;
        long long ans = 0;
        priority_queue<int, vector<int>, greater<> > pq1, pq2;
        for (; k > 0; --k) {
            while (pq1.size() < can && i <= j) {
                pq1.push(costs[i++]);
            }
            while (pq2.size() < can && j >= i) {
                pq2.push(costs[j--]);
            }

            int a = pq1.size() > 0 ? pq1.top() : INT_MAX;  // to avoid tle and check if a exists if not than update it
                                                           // to a very large value
            int b = pq2.size() > 0 ? pq2.top() : INT_MAX;

            if (a <= b) {
                ans += a;
                pq1.pop();
            } else {
                ans += b;
                pq2.pop();
            }
        }
        return ans;
    }
};