#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    void help(vector<int>& candidates, int target, vector<vector<int>>& ans, vector<int>& path, int ind) {
        if (target == 0) {
            ans.push_back(path);
            return;
        }

        for (int i = ind; i < static_cast<int>(candidates.size()); i++) {
            if (i > ind && candidates[i] == candidates[i - 1]) {
                continue;
            }

            if (candidates[i] > target) {
                break;
            }

            path.push_back(candidates[i]);

            help(candidates, target - candidates[i], ans, path, i + 1);

            path.pop_back();
        }
    }

    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        vector<vector<int>> ans;
        vector<int> path;
        help(candidates, target, ans, path, 0);
        return ans;
    }
};