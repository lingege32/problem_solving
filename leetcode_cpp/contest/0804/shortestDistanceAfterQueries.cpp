#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    vector<int> shortestDistanceAfterQueries(int n, vector<vector<int>>& queries) {
        unordered_map<int, vector<int>> edge;
        for (int i = 1; i < n; ++i) {
            edge[i - 1].push_back(i);
        }
        vector<int> dis(n, 0);
        for (int i = 0; i < n; ++i) {
            dis[i] = i;
        }
        vector<int> ret;
        ret.reserve(n);
        for (const auto& p : queries) {
            update(dis, p[0], p[1], edge);
            // for (auto d : dis) {
            //     std::cout << d << ", ";
            // }
            // std::cout << std::endl;
            ret.push_back(dis.back());
        }
        return ret;
    }

    void update(vector<int>& dis, int f, int t, unordered_map<int, vector<int>>& edge) {
        edge[f].push_back(t);
        std::queue<int> q;
        auto d = dis[f] + 1;
        q.push(t);
        while (!q.empty()) {
            int sz = q.size();
            for (int _i = 0; _i < sz; ++_i) {
                auto n = q.front();
                q.pop();
                if (dis[n] <= d) {
                    continue;
                }
                dis[n] = d;
                for (auto nn : edge[n]) {
                    q.push(nn);
                }
            }
            d++;
        }
    }
};