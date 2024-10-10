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
    static int findCircleNum(vector<vector<int>>& isConnected) {
        int ret = 0;
        std::vector<bool> visited(isConnected.size(), false);
        std::vector<bool> marked(isConnected.size(), false);
        for (size_t i = 0; i < isConnected.size(); ++i) {
            if (!marked[i]) {
                ret++;
            }
            inner(isConnected, visited, marked, i);
        }
        return ret;
    }

    static void inner(vector<vector<int>>& isConnected, vector<bool>& visited, vector<bool>& marked, int city) {
        visited[city] = true;
        marked[city] = true;
        auto& next_cities = isConnected[city];
        for (size_t i = 0; i < next_cities.size(); ++i) {
            if (next_cities[i] == 1 && !visited[i] && !marked[i]) {
                inner(isConnected, visited, marked, i);
            }
        }
        visited[city] = false;
    }
};