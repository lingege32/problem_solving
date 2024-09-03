#include <bits/stdc++.h>
using namespace std;
static const int __ = []() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 0;
}();

struct TwoDArray {
    size_t row;
    size_t col;
    std::vector<int> data;
    TwoDArray(int r, int c) : row(r), col(c), data(r * c, 0) {}
    int *operator[](size_t r) { return &data[r * col]; }
};
class Solution {
  public:
    int minimumSum(vector<vector<int>> &grid) {
        size_t left = grid[0].size();
        size_t right = 0;
        size_t top = 0;
        size_t bottom = grid.size();
        for (size_t r = 0; r < grid.size(); ++r) {
            for (size_t c = 0; c < grid[0].size(); ++c) {
                if (grid[r][c] == 1) {
                    left = std::min(left, c);
                    right = std::max(right, c);
                    top = std::max(top, r);
                    bottom = std::min(bottom, r);
                }
            }
        }
        vector<vector<int>> new_grid(top - bottom + 1,
                                     vector<int>(right - left + 1, 0));
        for (size_t r = bottom; r <= top; ++r) {
            for (size_t c = left; c <= right; ++c) {
                new_grid[r - bottom][c - left] = grid[r][c];
            }
        }
        return inner(new_grid);
    }
    int inner(vector<vector<int>> &grid) {
        const int m = grid.size(), n = grid[0].size();
        TwoDArray tl{m, n};
        TwoDArray tr{m, n};
        TwoDArray bl{m, n};
        TwoDArray br{m, n};
        TwoDArray h{m, m};
        TwoDArray v{n, n};
        int l, r, t, b;

        std::vector<int> top(n, 0), bottom(n, 0);
        for (int j = 0; j < n; ++j) {
            top[j] = -1;
        }
        for (int i = 0; i < m; ++i) {
            l = -1;
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    if (top[j] == -1) {
                        top[j] = i;
                    }
                    bottom[j] = i;
                }
                if (top[j] >= 0) {
                    if (l == -1) {
                        l = j, r = l;
                        t = top[j];
                        b = bottom[j];
                    } else {
                        r = j;
                        t = min(t, top[j]);
                        b = max(b, bottom[j]);
                    }
                }
                tl[i][j] = (l == -1) ? 0 : (r - l + 1) * (b - t + 1);
            }
        }
        for (int j = 0; j < n; ++j) {
            top[j] = -1;
        }
        for (int i = 0; i < m; ++i) {
            l = -1;
            for (int j = n - 1; j >= 0; --j) {
                if (grid[i][j] == 1) {
                    if (top[j] == -1) {
                        top[j] = i;
                    }
                    bottom[j] = i;
                }
                if (top[j] >= 0) {
                    if (l == -1) {
                        l = j, r = l;
                        t = top[j], b = bottom[j];
                    } else {
                        l = j;
                        t = min(t, top[j]);
                        b = max(b, bottom[j]);
                    }
                }
                tr[i][j] = (l == -1) ? 0 : (r - l + 1) * (b - t + 1);
            }
        }
        for (int j = 0; j < n; ++j) {
            top[j] = -1;
        }
        for (int i = m - 1; i >= 0; --i) {
            l = -1;
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    if (top[j] == -1) {
                        bottom[j] = i;
                    }
                    top[j] = i;
                }
                if (top[j] >= 0) {
                    if (l == -1) {
                        l = j, r = l;
                        t = top[j], b = bottom[j];
                    } else {
                        r = j;
                        t = min(t, top[j]);
                        b = max(b, bottom[j]);
                    }
                }
                bl[i][j] = (l == -1) ? 0 : (r - l + 1) * (b - t + 1);
            }
        }
        for (int j = 0; j < n; ++j) {
            top[j] = -1;
        }
        for (int i = m - 1; i >= 0; --i) {
            l = -1;
            for (int j = n - 1; j >= 0; --j) {
                if (grid[i][j] == 1) {
                    if (top[j] == -1) {
                        bottom[j] = i;
                    }
                    top[j] = i;
                }
                if (top[j] >= 0) {
                    if (l == -1) {
                        l = j, r = l;
                        t = top[j], b = bottom[j];
                    } else {
                        l = j;
                        t = min(t, top[j]);
                        b = max(b, bottom[j]);
                    }
                }
                br[i][j] = (l == -1) ? 0 : (r - l + 1) * (b - t + 1);
            }
        }
        TwoDArray hends{m, 2}, vends{n, 2};
        for (int j = 0; j < n; ++j) {
            vends[j][0] = -1;
        }
        for (int i = 0; i < m; ++i) {
            hends[i][0] = -1;
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    if (hends[i][0] == -1) {
                        hends[i][0] = j, hends[i][1] = j;
                    } else {
                        hends[i][1] = j;
                    }
                    if (vends[j][0] == -1) {
                        vends[j][0] = i, vends[j][1] = i;
                    } else {
                        vends[j][1] = i;
                    }
                }
            }
        }
        for (int i = 0; i < m; ++i) {
            l = -1;
            for (int j = i; j >= 0; --j) {
                if (hends[j][0] >= 0) {
                    if (l == -1) {
                        l = hends[j][0];
                        r = hends[j][1];
                        t = j, b = j;
                    } else {
                        l = min(l, hends[j][0]);
                        r = max(r, hends[j][1]);
                        t = j;
                    }
                }
                h[j][i] = (l == -1) ? 0 : (r - l + 1) * (b - t + 1);
            }
        }
        for (int i = 0; i < n; ++i) {
            l = -1;
            for (int j = i; j >= 0; --j) {
                if (vends[j][0] >= 0) {
                    if (l == -1) {
                        t = vends[j][0];
                        b = vends[j][1];
                        l = j, r = j;
                    } else {
                        t = min(t, vends[j][0]);
                        b = max(b, vends[j][1]);
                        l = j;
                    }
                }
                v[j][i] = (l == -1) ? 0 : (r - l + 1) * (b - t + 1);
            }
        }
        int minarea = m * n;
        for (int i = 0; i < m; ++i) {
            for (int j = i + 1; j < m - 1; ++j) {
                if (h[0][i] + h[i + 1][j] + h[j + 1][m - 1] < minarea) {
                    minarea = h[0][i] + h[i + 1][j] + h[j + 1][m - 1];
                }
            }
        }
        for (int i = 0; i < n; ++i) {
            for (int j = i + 1; j < n - 1; ++j) {
                if (v[0][i] + v[i + 1][j] + v[j + 1][n - 1] < minarea) {
                    minarea = v[0][i] + v[i + 1][j] + v[j + 1][n - 1];
                }
            }
        }
        for (int i = 0; i < m - 1; ++i) {
            for (int j = 0; j < n - 1; ++j) {
                minarea =
                    min({minarea, tl[i][j] + bl[i + 1][j] + v[j + 1][n - 1],
                         v[0][j] + tr[i][j + 1] + br[i + 1][j + 1],
                         h[i + 1][m - 1] + tl[i][j] + tr[i][j + 1],
                         h[0][i] + bl[i + 1][j] + br[i + 1][j + 1]});
            }
        }
        return minarea;
    }
};