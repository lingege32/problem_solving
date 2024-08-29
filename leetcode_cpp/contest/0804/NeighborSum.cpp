#include <bits/stdc++.h>
using namespace std;

class NeighborSum {
    vector<vector<std::pair<int, int>>> g;
    vector<std::pair<int,int>> mapping;

  public:
    NeighborSum(vector<vector<int>>& grid) {
        size_t len = grid.size();
        mapping = vector<std::pair<int, int>>(len * len, std::make_pair(0,0));
        g = vector<vector<std::pair<int, int>>>(len, vector<std::pair<int, int>>(len, std::make_pair(0, 0)));
        for (int x = 0; x < len; ++x) {
            for (int y = 0; y < len; ++y) {
                auto v = grid[x][y];
                mapping[v] = std::make_pair(x,y);
                auto x1 = x - 1;
                auto x2 = x + 1;
                auto y1 = y - 1;
                auto y2 = y + 1;
                if (x1 >= 0) {
                    if (y1 >= 0) {
                        g[x1][y1].first += v;
                    }
                    g[x1][y].second += v;
                    if (y2 < len) {
                        g[x1][y2].first += v;
                    }
                }
                if (y1 >= 0) {
                    g[x][y1].second += v;
                }
                if (y2 < len) {
                    g[x][y2].second += v;
                }
                if (x2 < len) {
                    if (y1 >= 0) {
                        g[x2][y1].first += v;
                    }
                    g[x2][y].second += v;
                    if (y2 < len) {
                        g[x2][y2].first += v;
                    }
                }
            }
        }
    }

    int adjacentSum(int value) { return g[mapping[value].first][mapping[value].second].second; }

    int diagonalSum(int value) { return g[mapping[value].first][mapping[value].second].first; }
};

/**
 * Your NeighborSum object will be instantiated and called as such:
 * NeighborSum* obj = new NeighborSum(grid);
 * int param_1 = obj->adjacentSum(value);
 * int param_2 = obj->diagonalSum(value);
 */