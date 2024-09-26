#include <bits/stdc++.h>

using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
    static auto createCondition(const vector<vector<int>>& cond, int n) {
        vector<pair<int, unordered_set<int>>> ret(n, {0, unordered_set<int>{}});
        for (const auto& c : cond) {
            auto f = c[0];
            auto t = c[1];
            auto it = ret[f].second.insert(t);
            if (it.second) {
                ret[t].first++;
            }
        }
        return ret;
    }

    static std::optional<vector<vector<int>>> levelization(const vector<vector<int>>& conditions, int n) {
        auto cond = createCondition(conditions, n);
        std::vector<vector<int>> levels;
        std::vector<int> cur_level;
        for (int i = 1; i < n; ++i) {
            if (cond[i].first == 0 && !cond[i].second.empty()) {
                cur_level.push_back(i);
            }
        }

        while (!cur_level.empty()) {
            std::vector<int> next;
            for (auto i : cur_level) {
                for (auto adj : cond[i].second) {
                    if (--cond[adj].first == 0) {
                        next.push_back(adj);
                    }
                }
            }
            levels.push_back(std::move(cur_level));
            cur_level = std::move(next);
        }

        if (std::ranges::any_of(cond, [](const auto& p) { return p.first != 0; })) {
            return std::nullopt;
        }

        return levels;
    }

    static std::vector<pair<int, int>> createCoor(const vector<vector<int>>& row, const vector<vector<int>>& col,
                                                  int n) {
        std::vector<pair<int, int>> ret(n, {-1, -1});
        int rowS = row.size();
        int colS = col.size();
        int level = 0;
        for (int i = 0; i < rowS; ++i) {
            for (auto v : row[i]) {
                ret[v].first = level++;
            }
        }
        for (int i = 1; i < n; ++i) {
            if (ret[i].first == -1) {
                ret[i].first = level++;
            }
        }
        level = 0;
        for (int i = 0; i < colS; ++i) {
            for (auto v : col[i]) {
                ret[v].second = level++;
            }
        }
        for (int i = 1; i < n; ++i) {
            if (ret[i].second == -1) {
                ret[i].second = level++;
            }
        }
        return ret;
    }

  public:
    static vector<vector<int>> buildMatrix(int k, vector<vector<int>>& rowConditions,
                                           vector<vector<int>>& colConditions) {
        int n = k + 1;
        auto rowLevel = levelization(rowConditions, n);
        if (!rowLevel) {
            return {};
        }
        auto colLevel = levelization(colConditions, n);
        if (!colLevel) {
            return {};
        }

        auto coor = createCoor(rowLevel.value(), colLevel.value(), n);
        vector<vector<int>> ret(k, vector<int>(k, 0));
        for (int i = 1; i < n; ++i) {
            auto r = coor[i].first;
            auto c = coor[i].second;
            ret[r][c] = i;
        }
        return ret;
    }
};