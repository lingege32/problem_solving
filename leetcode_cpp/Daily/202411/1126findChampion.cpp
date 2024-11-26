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
    static int findChampion(int n, vector<vector<int>>& edges) {
        std::vector<int> incnt(n,0);
        for (const auto& edge : edges) {
            incnt[edge[1]]++;
        }
        int ret = -1;
        for (int i = 0; i < n; ++i) {
            if (incnt[i] == 0) {
                if (ret != -1) {
                    return -1;
                }
                ret = i;
            }
        }
        return ret;
    }
};