#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArray {
    using Self = TwoDArray;
    std::array<int, 6> data;

    explicit TwoDArray(const vector<vector<int>>& v) {
        int k = 0;
        for (int i = 0; i < 2; i++) {
            for (int j = 0; j < 3; j++) {
                data[k++] = v[i][j];
            }
        }
    }

    int* operator[](int i) { return &data[i * 3]; }

    [[nodiscard]] bool isDone() const {
        return data[0] == 1 && data[1] == 2 && data[2] == 3 && data[3] == 4 && data[4] == 5 && data[5] == 0;
    }

    bool operator==(const Self& other) const { return data == other.data; }

    void applyNew(std::vector<Self>& q) {
        int i = 0, j = 0;
        for (int ii = 0; ii < 2; ++ii) {
            for (int jj = 0; jj < 3; ++jj) {
                if (data[(ii * 3) + jj] == 0) {
                    i = ii;
                    j = jj;
                    break;
                }
            }
        }
        if (i == 1) {
            swap(i, j, i - 1, j);
            q.push_back(*this);
            swap(i, j, i - 1, j);
        } else {
            swap(i, j, i + 1, j);
            q.push_back(*this);
            swap(i, j, i + 1, j);
        }
        if (j != 0) {
            swap(i, j, i, j - 1);
            q.push_back(*this);
            swap(i, j, i, j - 1);
        }
        if (j != 2) {
            swap(i, j, i, j + 1);
            q.push_back(*this);
            swap(i, j, i, j + 1);
        }
    }

    void swap(int i, int j, int ni, int nj) { std::swap(operator[](i)[j], operator[](ni)[nj]); }
};

template <>
struct std::hash<TwoDArray> {
    std::size_t operator()(TwoDArray const& s) const noexcept {
        std::size_t seed = s.data.size();
        for (auto& i : s.data) {
            seed ^= i + 0x9e3779b9 + (seed << 6) + (seed >> 2);
        }
        return seed;
    }
};

class Solution {
  public:
    static int slidingPuzzle(vector<vector<int>>& board) {
        TwoDArray array(board);
        std::unordered_set<TwoDArray> visited;
        std::vector<TwoDArray> q;
        q.push_back(array);
        int depth = 0;
        while (!q.empty()) {
            std::vector<TwoDArray> newQ;
            for (auto& arr : q) {
                auto [it, insert] = visited.insert(arr);
                if (!insert) {
                    continue;
                }
                if (arr.isDone()) {
                    return depth;
                }

                arr.applyNew(newQ);
            }
            std::swap(newQ, q);
            depth++;
        }
        return -1;
    }
};