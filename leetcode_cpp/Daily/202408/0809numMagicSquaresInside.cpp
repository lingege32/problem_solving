#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static bool isMagic(int i, int j, vector<vector<int>>& grid) {
        //    if (grid[i][j] != 5) return 0;

        bitset<10> once;  // bitset of size 10 to track digits 1-9
        array<int, 3> rowSum, colSum;
        rowSum.fill(0);
        colSum.fill(0);

        for (int a = i - 1; a <= i + 1; a++) {
            for (int b = j - 1; b <= j + 1; b++) {
                int x = grid[a][b];
                if (x < 1 || x > 9) {
                    return false;  // invalid number for a magic square
                }
                rowSum[a - i + 1] += x;
                colSum[b - j + 1] += x;
                once.flip(x);
            }
        }
        // numbers 1-9 are present exactly once
        if (once.count() != 9) {
            return false;
        }

        // all rows and columns sum to 15
        if (!std::ranges::all_of(rowSum, [](int sum) { return sum == 15; })) {
            return false;
        }
        if (!std::ranges::all_of(colSum, [](int sum) { return sum == 15; })) {
            return false;
        }

        // Check if diagonals sum to 15
        return (grid[i - 1][j - 1] + grid[i + 1][j + 1] == 10 && grid[i + 1][j - 1] + grid[i - 1][j + 1] == 10);
    }

    static int numMagicSquaresInside(vector<vector<int>>& grid) {
        const int r = grid.size(), c = grid[0].size();
        if (r < 3 || c < 3) {
            return 0;
        }

        int cnt = 0;
        for (int i = 1; i < r - 1; i++) {
            for (int j = 1; j < c - 1; j++) {
                if (grid[i][j] == 5) {
                    cnt += isMagic(i, j, grid);
                }
            }
        }
        return cnt;
    }
};