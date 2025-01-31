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
    static int maximumInvitations(vector<int>& favorites) {
        int n = favorites.size();
        vector<int> inDegree(n, 0), chainLengths(n, 0);
        vector<bool> visited(n, false);
        for (int in = 0; in < n; ++in) {
            inDegree[favorites[in]]++;
        }

        // find the start point
        std::queue<int> q;
        for (int in = 0; in < n; ++in) {
            if (inDegree[in] == 0) {
                q.push(in);
            }
        }

        while (!q.empty()) {
            int in = q.front();
            q.pop();
            visited[in] = true;
            int out = favorites[in];
            if (--inDegree[out] == 0) {
                q.push(out);
            }
            chainLengths[out] = chainLengths[in] + 1;
        }

        int maxCycle = 0;
        int totalLength = 0;
        for (int i = 0; i < n; ++i) {
            if (visited[i]) {
                continue;
            }

            int cycle = 0;
            int cur = i;
            while (!visited[cur]) {
                cycle++;
                visited[cur] = true;
                cur = favorites[cur];
            }
            if (cycle == 2) {
                // we can see they are one edge and can connected to single list
                // 1 --> 2 --> 3 --> 4
                //       ^-----|

                totalLength += (2 + chainLengths[i] + chainLengths[favorites[i]]);
            } else {
                maxCycle = std::max(maxCycle, cycle);
            }
        }
        return std::max(maxCycle, totalLength);
    }
};