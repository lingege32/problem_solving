#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    static vector<int> resultsArray(vector<vector<int>>& queries, int k) {
        const int n = queries.size();
        // Max heap to keep track of the smallest k elements
        priority_queue<int> pq;
        vector<int> ans(n, -1);

        for (int i = 0; i < n; i++) {
            int x = abs(queries[i][0])+abs(queries[i][1]);
            pq.push(x);  

            // If heap size exceeds k, remove the largest element
            if (pq.size() > k) 
                pq.pop();

            if (pq.size() == k) {
                ans[i] = pq.top();
            }
        }

        return ans;
    }
};