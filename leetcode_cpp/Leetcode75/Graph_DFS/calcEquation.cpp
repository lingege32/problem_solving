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
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values,
                                vector<vector<string>>& queries) {
        std::unordered_map<std::string, std::unordered_map<std::string, double>> mapping;
        int n = equations.size();
        for (int i = 0; i < n; ++i) {
            auto& equa = equations[i];
            auto val = values[i];
            mapping[equa[0]][equa[1]] = val;
            mapping[equa[1]][equa[0]] = 1 / val;
            mapping[equa[0]][equa[0]] = val;
            mapping[equa[1]][equa[1]] = val;
        }
        std::vector<double> ret;
        ret.reserve(queries.size());
        for (auto& query : queries) {
            double ret_double = -1.0;
            std::unordered_set<std::string> visited;
            auto& t = query[0];
            auto& d = query[1];
            if (mapping.contains(t) && mapping.contains(d)) {
                dfs(query[0], query[1], 1.0, mapping, visited, ret_double);
            }
            if (ret_double != -1.0) {
                mapping[query[0]][query[1]] = ret_double;
                mapping[query[1]][query[0]] = 1 / ret_double;
            }
            ret.push_back(ret_double);
        }
        return ret;
    }

    void dfs(const std::string& cur, const std::string& target, double val,
             std::unordered_map<std::string, std::unordered_map<std::string, double>>& mapping,
             std::unordered_set<std::string>& visited, double& ret) {
        if (ret != -1.0) {
            return;
        }
        if (cur == target) {
            ret = val;
            return;
        }
        if (!visited.insert(cur).second) {
            return;
        }
        auto it = mapping.find(cur);
        if (it == mapping.end()) {
            return;
        }
        for (auto& [next, next_val] : it->second) {
            dfs(next, target, val * next_val, mapping, visited, ret);
        }
    }
};