#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

template <typename T, size_t N>
struct TwoDArray {
    std::array<T, N * N> data;

    template <typename U>
    void fill(U&& val) {
        data.fill(std::forward<U>(val));
    }

    T* operator[](size_t idx) { return &data[N * idx]; }
};

constexpr long long INVALID_COST = std::numeric_limits<long long>::max();

class Solution {
  public:
    static TwoDArray<long long, 26> costTable(const vector<char>& original, const vector<char>& changed,
                                              const vector<int>& cost) {
        TwoDArray<long long, 26> edgesCost;
        edgesCost.fill(INVALID_COST);
        for (size_t idx = 0; idx < original.size(); ++idx) {
            edgesCost[original[idx] - 'a'][changed[idx] - 'a'] = std::min(
                edgesCost[original[idx] - 'a'][changed[idx] - 'a'], static_cast<long long>(cost[idx]));
        }

        TwoDArray<long long, 26> dist;
        dist.fill(INVALID_COST);
        for (size_t i = 0; i < 26; ++i) {
            dist[i][i] = 0;
        }

        for (size_t k = 0; k < 26; ++k) {
            std::queue<size_t> q;
            q.push(k);
            while (!q.empty()) {
                size_t i = q.front();
                q.pop();

                auto f = dist[k][i];
                for (size_t j = 0; j < 26; ++j) {
                    auto c = edgesCost[i][j];
                    if (c == INVALID_COST) {
                        continue;
                    }
                    auto next = c + f;
                    auto& min_distance = dist[k][j];
                    if (next < min_distance) {
                        q.push(j);
                        min_distance = next;
                    }
                }
            }
        }

        return dist;
    }

    static long long minimumCost(string source, string target, vector<char>& original, vector<char>& changed,
                                 vector<int>& cost) {
        auto table = costTable(original, changed, cost);
        long long ans = 0;
        for (size_t idx = 0; idx < source.size(); ++idx) {
            auto c1 = source[idx];
            auto c2 = target[idx];
            if (c1 == c2) {
                continue;
            }
            auto v = table[c1 - 'a'][c2 - 'a'];
            if (v == INVALID_COST) {
                return -1;
            }
            ans += v;
        }
        return ans;
    }
};