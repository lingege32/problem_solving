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