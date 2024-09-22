#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

// https://hyper-meta.blogspot.com/
int adjl[10000][10000], asz[10000];
uint layerOdd[10000], layerEven[10000], sz;
queue<int> q;

class Solution {
    static void bfs() {
        layerEven[0] = 0;
        q.emplace(0);
        while (!q.empty()) {
            int cur = q.front();
            for (int j = 0; j < asz[cur]; ++j) {
                int nei = adjl[cur][j];
                if (layerOdd[cur] < layerEven[nei] - 1) {
                    layerEven[nei] = layerOdd[cur] + 1, q.emplace(nei);
                }
                if (layerEven[cur] < layerOdd[nei] - 1) {
                    layerOdd[nei] = layerEven[cur] + 1, q.emplace(nei);
                }
            }
            q.pop();
        }
    }

  public:
    static int secondMinimum(int n, vector<vector<int>>& edges, int time, int change) {
        sz = n;
        memset(asz, 0, sz * sizeof(asz[0]));
        for (const auto& v : edges) {
            int a = v[0] - 1, b = v[1] - 1;
            adjl[a][asz[a]++] = b;
            adjl[b][asz[b]++] = a;
        }
        memset(layerOdd, -1, sz * sizeof(layerOdd[0]));
        memset(layerEven, -1, sz * sizeof(layerEven[0]));
        bfs();
        int t = 0, c2 = change << 1;
        if (layerOdd[sz - 1] < layerEven[sz - 1]) {
            int mind = min(layerOdd[sz - 1] + 2, layerEven[sz - 1]);
            for (int i = 0; i < mind - 1; ++i) {
                t += time;
                int m = t % c2;
                t += (m >= change) * (c2 - m);
            }
        } else {
            int mind = min(layerOdd[sz - 1], layerEven[sz - 1] + 2);
            for (int i = 0; i < mind - 1; ++i) {
                t += time;
                int m = t % c2;
                t += (m >= change) * (c2 - m);
            }
        }
        return t + time;
    }
};

class EasyToReadSolution {
  public:
    using int2 = pair<int, int>;

    static int wtime(int step, int time, int change) {
        int ans = 0;
        for (int i = 0; i < step; i++) {
            int gr = ans / change;
            if (gr & 1) {  // If it's a red light
                ans = (gr + 1) * change;
            }
            ans += time;
        }
        return ans;
    }

    static int secondMinimum(int n, vector<vector<int>>& edges, int time, int change) {
        vector<vector<int>> adj(n + 1);
        for (auto& e : edges) {
            int v = e[0], w = e[1];
            adj[v].push_back(w);
            adj[w].push_back(v);
        }

        vector<int> dist(n + 1, INT_MAX), dist2(n + 1, INT_MAX);
        queue<int2> qq;  // {node, distance}
        qq.emplace(1, 0);
        dist[1] = 0;

        while (!qq.empty()) {
            auto [x, d] = qq.front();
            qq.pop();
            for (int y : adj[x]) {
                int newD = d + 1;
                if (newD < dist[y]) {
                    dist2[y] = dist[y];
                    dist[y] = newD;
                    qq.emplace(y, newD);
                } else if (newD > dist[y] && newD < dist2[y]) {
                    dist2[y] = newD;
                    qq.emplace(y, newD);
                }
            }
        }

        int steps = dist2[n];
        if (steps == INT_MAX) {
            return -1;
        }
        return wtime(steps, time, change);
    }
};
