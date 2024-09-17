#include <bits/stdc++.h>
using namespace std;

class Solution {
  private:
    std::array<int, 4> dx = {-1, 1, 0, 0};
    std::array<int, 4> dy = {0, 0, -1, 1};

    void dfs(int i, int j, int n, int m, vector<vector<int>> &grid, vector<vector<int>> &vis) {
        vis[i][j] = 1;

        for (int k = 0; k < 4; k++) {
            int newx = i + dx[k];
            int newy = j + dy[k];

            if (newx >= 0 and newy >= 0 and newx < n and newy < m and grid[newx][newy] == 1 and vis[newx][newy] == 0) {
                dfs(newx, newy, n, m, grid, vis);
            }
        }
    }

  private:
    int numIslands(vector<vector<int>> &grid) {
        int n = grid.size();
        int m = grid[0].size();

        vector<vector<int>> vis(n, vector<int>(m, 0));  // no is visited initially
        int noOfIslands = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (vis[i][j] == 0 and grid[i][j] == 1) {
                    dfs(i, j, n, m, grid, vis);
                    noOfIslands++;
                }
            }
        }

        return noOfIslands;
    }

  private:
    bool solve(int node, int parent, int &timer, vector<int> &low, vector<int> &tin, vector<int> &vis,
               vector<vector<int>> &adj) {
        vis[node] = 1;
        low[node] = tin[node] = timer++;
        int child = 0;

        for (auto adjNode : adj[node]) {
            if (adjNode == parent) {
                continue;
            }

            if (!vis[adjNode]) {
                if (solve(adjNode, node, timer, low, tin, vis, adj)) {
                    return true;
                }

                low[node] = min(low[node], low[adjNode]);
                if (low[adjNode] >= tin[node] and parent != -1) {
                    return true;
                }
                child++;
            } else {
                low[node] = min(low[node], tin[adjNode]);
            }
        }
        return parent == -1 and child > 1;
    }

  private:
    void make_graph(vector<vector<int>> &grid, vector<vector<int>> &adj, int &cnt) {
        int n = grid.size();
        int m = grid[0].size();

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == 1) {
                    cnt++;

                    for (int k = 0; k < 4; k++) {
                        int newx = i + dx[k];
                        int newy = j + dy[k];

                        if (newx >= 0 and newy >= 0 and newx < n and newy < m and grid[newx][newy] == 1) {
                            adj[i * m + j].push_back(newx * m + newy);
                        }
                    }
                }
            }
        }
    }

  public:
    int minDays(vector<vector<int>> &grid) {
        int n = grid.size();
        int m = grid[0].size();

        int totalIslands = numIslands(grid);

        // base case
        if (totalIslands > 1 or totalIslands == 0) {
            return 0;
        }

        vector<vector<int>> adj(n * m);
        int cnt = 0;
        make_graph(grid, adj, cnt);

        // base case
        if (cnt == 1) {
            return 1;
        }

        vector<int> vis(n * m, 0), tin(n * m, 0), low(n * m, 0);
        int timer = 0;

        for (int i = 0; i < n * m; i++) {
            if (!vis[i]) {
                if (solve(i, -1, timer, low, tin, vis, adj)) {
                    return 1;
                }
            }
        }

        return 2;
    }
};