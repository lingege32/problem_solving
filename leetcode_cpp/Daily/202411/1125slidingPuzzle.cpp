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

struct Dict {
    static inline std::unordered_map<TwoDArray, int> mapping;
    static void bfs() {
        if (!mapping.empty()) {
            return;
        }
        TwoDArray root({{1, 2, 3}, {4, 5, 0}});
        std::vector<TwoDArray> q;
        int depth = 0;
        q.push_back(root);
        while (!q.empty()) {
            std::vector<TwoDArray> newQ;
            for (auto& arr : q) {
                auto [it, insert] = mapping.emplace(arr, depth);
                if (!insert) {
                    continue;
                }
                arr.applyNew(newQ);
            }
            std::swap(newQ, q);
            depth++;
        }
    }

    static int get(const TwoDArray& arr) {
        auto it = mapping.find(arr);
        return it == mapping.end() ? -1 : it->second;
    }

};

class Solution {
  public:
    static int slidingPuzzle(vector<vector<int>>& board) {
        Dict::bfs();
        return Dict::get(TwoDArray(board));
    }
};