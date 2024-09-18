#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static vector<vector<int>> spiralMatrixIII(int rows, int cols, int rStart, int cStart) {
        vector<vector<int>> ret;
        auto push = [&](int r, int c) { ret.emplace_back(std::vector<int>{r, c}); };
        push(rStart, cStart);
        int count = (rows * cols) - 1;
        int step = 1;
        while (count > 0) {
            for (int i = 0; i < step; i++) {
                cStart++;
                if (rStart >= 0 && rStart < rows && cStart >= 0 && cStart < cols) {
                    push(rStart, cStart);
                    count--;
                }
            }
            for (int i = 0; i < step; i++) {
                rStart++;
                if (rStart >= 0 && rStart < rows && cStart >= 0 && cStart < cols) {
                    push(rStart, cStart);
                    count--;
                }
            }
            step++;
            for (int i = 0; i < step; i++) {
                cStart--;
                if (rStart >= 0 && rStart < rows && cStart >= 0 && cStart < cols) {
                    push(rStart, cStart);
                    count--;
                }
            }
            for (int i = 0; i < step; i++) {
                rStart--;
                if (rStart >= 0 && rStart < rows && cStart >= 0 && cStart < cols) {
                    push(rStart, cStart);
                    count--;
                }
            }
            step++;
        }

        return ret;
    }
};