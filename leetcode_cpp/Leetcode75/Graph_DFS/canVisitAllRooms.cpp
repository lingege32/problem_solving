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
    bool canVisitAllRooms(vector<vector<int>>& rooms) {
        std::vector<bool> visited(rooms.size(), false);
        std::vector<bool> ans(rooms.size(), false);
        inner(rooms, visited, ans, 0);
        return std::ranges::all_of(ans, [](bool v) { return v; });
    }

    void inner(vector<vector<int>>& rooms, vector<bool>& visited, vector<bool>& ans, int room) {
        ans[room] = true;
        visited[room] = true;

        for (auto r : rooms[room]) {
            if (!visited[r] && !ans[r]) {
                inner(rooms, visited, ans, r);
            }
        }
        visited[room] = false;
    }
};