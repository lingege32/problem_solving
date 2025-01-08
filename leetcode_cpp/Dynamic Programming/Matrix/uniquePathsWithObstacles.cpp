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
    static int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
        if (obstacleGrid[0][0] == 1) {
            return 0;
        }
        for (auto& row : obstacleGrid) {
            for (auto& cell : row) {
                if (cell == 1) {
                    cell = -1;
                }
            }
        }
        int row = obstacleGrid.size();
        int col = obstacleGrid[0].size();
        obstacleGrid[0][0] = 1;
        for (int i = 1; i < col; ++i) {
            if (obstacleGrid[0][i] == -1) {
                break;
            }
            obstacleGrid[0][i] = 1;
        }
        for (int i = 1; i < row; ++i) {
            if (obstacleGrid[i][0] == -1) {
                break;
            }
            obstacleGrid[i][0] = 1;
        }
        for (int i = 1; i < row; ++i) {
            for (int j = 1; j < col; ++j) {
                if (obstacleGrid[i][j] != -1) {
                    obstacleGrid[i][j] = std::max(obstacleGrid[i - 1][j], 0) + std::max(obstacleGrid[i][j - 1], 0);
                }
            }
        }
        return std::max(obstacleGrid.back().back(), 0);
    }
};