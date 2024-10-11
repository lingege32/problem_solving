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
    static int nearestExit(vector<vector<char>>& maze, vector<int>& entrance) {
        std::vector<std::pair<int, int>> stack;
        std::vector<std::pair<int, int>> tmp;
        int row = maze.size();
        int col = maze[0].size();
        stack.emplace_back(entrance[0], entrance[1]);
        int level = 0;
        while (!stack.empty()) {
            for (const auto& [x, y] : stack) {
                if (x < 0 || y < 0 || x == row || y == col) {
                    continue;
                }
                auto& m = maze[x][y];
                if (m == '+') {
                    continue;
                }
                if (level != 0 && (x == 0 || y == 0 || x == row - 1 || y == col - 1)) {
                    return level;
                }
                m = '+';
                tmp.emplace_back(x - 1, y);
                tmp.emplace_back(x + 1, y);
                tmp.emplace_back(x, y - 1);
                tmp.emplace_back(x, y + 1);
            }
            level++;
            stack.swap(tmp);
            tmp.clear();
        }

        return -1;
    }
};