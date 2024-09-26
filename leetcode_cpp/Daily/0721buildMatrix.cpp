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
        vector<pair<int, vector<int>>> ret(n, {0, vector<int>{}});
        for (const auto& c : cond) {
            auto f = c[0];
            auto t = c[1];
            ret[f].second.push_back(t);
            // if (it.second) {
            ret[t].first++;
            // }
        }
        return ret;
    }

    static std::optional<vector<int>> levelization(const vector<vector<int>>& conditions, int n) {
        auto cond = createCondition(conditions, n);
        std::vector<int> nLevel(n, -1);
        std::vector<int> cur_level;
        for (int i = 1; i < n; ++i) {
            if (cond[i].first == 0 && !cond[i].second.empty()) {
                cur_level.push_back(i);
            }
        }
        int level = 0;
        while (!cur_level.empty()) {
            std::vector<int> next;
            for (auto i : cur_level) {
                nLevel[i] = level++;
                for (auto adj : cond[i].second) {
                    if (--cond[adj].first == 0) {
                        next.push_back(adj);
                    }
                }
            }
            std::swap(cur_level, next);
        }

        if (std::ranges::any_of(cond, [](const auto& p) { return p.first != 0; })) {
            return std::nullopt;
        }
        for (int i = 1; i < n; ++i) {
            if (nLevel[i] == -1) {
                nLevel[i] = level++;
            }
        }
        return nLevel;
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
        vector<vector<int>> ret(k, vector<int>(k, 0));
        auto& rl = rowLevel.value();
        auto& cl = colLevel.value();
        for (int i = 1; i < n; ++i) {
            auto r = rl[i];
            auto c = cl[i];
            ret[r][c] = i;
        }
        return ret;
    }
};