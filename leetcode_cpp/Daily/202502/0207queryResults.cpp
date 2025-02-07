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
    static vector<int> queryResults(int /*limit*/, vector<vector<int>>& queries) {
        int n = queries.size();
        std::vector<int> ret;
        std::unordered_map<int, int> ball_to_color;
        std::unordered_map<int, int> color_count;
        int colorNum = 0;
        ret.reserve(n);
        for(auto& query: queries) {
            auto ballId = query[0];
            auto color = query[1];
            auto& ck = ball_to_color[ballId];
            if (ck) {
                colorNum -= !(--color_count[ck]);
            }
            ck = color;
            if (!color_count[color]++) {
                colorNum++;
            }

            ret.emplace_back(colorNum);
        }

        return ret;
    }
};