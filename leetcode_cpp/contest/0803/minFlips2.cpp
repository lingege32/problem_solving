#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int minFlips(vector<vector<int>> &A) {
        int N = A.size(), M = A[0].size();
        int Flips = 0, OneOne = 0, OneZero_ZeroOne = 0;

        for (int R = 0; R < (N + 1) / 2; R++) {
            for (int C = 0; C < (M + 1) / 2; C++) {
                int Ones = A[R][C], Cells = 1;

                if (R != N - R - 1) {
                    Ones += A[N - R - 1][C];
                    Cells += 1;
                    if (C != M - C - 1) {
                        Ones += A[N - R - 1][M - C - 1];
                        Cells += 1;
                    }
                }

                if (C != M - C - 1) {
                    Ones += A[R][M - C - 1];
                    Cells += 1;
                }

                if (Cells == 1) {
                    // if the mid point is one, and total is impossible to be 4x
                    Flips += Ones;
                } else if (Cells == 2) {
                    if (Ones == 1) {
                        Flips++, OneZero_ZeroOne = 1;
                    }
                    if (Ones == 2) {
                        OneOne++;
                    }
                } else if (Cells == 4) {
                    // 4 of 1 or 4 or 2 does not affect the constraint 4x
                    Flips += min(Ones, Cells - Ones);
                }
            }
        }

        if (OneOne % 2 == 1 && OneZero_ZeroOne == 0) {
            Flips += 2;
        }

        return Flips;
    }
};