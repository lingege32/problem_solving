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
    static int minReorder(int n, vector<vector<int>>& connections) {
        std::vector<std::vector<int>> connect1(n);
        std::vector<std::vector<int>> connect2(n);
        std::vector<bool> visited(n, false);
        for (auto& con : connections) {
            connect1[con[0]].push_back(con[1]);
            connect2[con[1]].push_back(con[0]);
        }
        int flip = 0;
        std::vector<int> stack{0};
        while (!stack.empty()) {
            auto t = stack.back();
            stack.pop_back();
            visited[t] = true;
            for (auto next : connect2[t]) {
                if (!visited[next]) {
                    flip++;
                    stack.push_back(next);
                }
            }
            connect2[t].clear();
            for (auto next : connect1[t]) {
                if (!visited[next]) {
                    stack.push_back(next);
                }
            }
        }
        return n - 1 - flip;
    }
};