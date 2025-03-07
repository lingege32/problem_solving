#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Algorithm {
    std::vector<std::vector<int>> adjacent;
    std::vector<int> amount;
    std::vector<int> unlockedByBob;
    int max = std::numeric_limits<int>::min();

    void traverseBob(int bob, std::vector<int>& path, std::vector<bool>& visited) {
        path.push_back(bob);
        if (bob == 0) {
            return;
        }
        visited[bob] = true;
        for (auto a : adjacent[bob]) {
            if (visited[a]) {
                continue;
            }
            traverseBob(a, path, visited);
            if (path.back() == 0) {
                return;
            }
        }
        path.pop_back();
        visited[bob] = false;
    }

    void traverseAlice(int level, int alice, int parent, int current) {
        auto& adj = adjacent[alice];
        int val = [&]() {
            if (level == unlockedByBob[alice]) {
                return amount[alice] / 2;
            }
            if (level > unlockedByBob[alice]) {
                return 0;
            }
            return amount[alice];
        }();
        current += val;
        if (adj.size() == 1 && alice != 0) {
            max = std::max(max, current);
            return;
        }

        for (auto a : adj) {
            if (a == parent) {
                continue;
            }
            traverseAlice(level + 1, a, alice, current);
        }
    }

  public:
    Algorithm(int bob, std::vector<std::vector<int>>& edges, std::vector<int> amount):
      amount{std::move(amount)} {
        int n = edges.size() + 1;
        adjacent.resize(n);
        for (auto& edge : edges) {
            adjacent[edge[0]].push_back(edge[1]);
            adjacent[edge[1]].push_back(edge[0]);
        }
        std::vector<bool> visited(n, false);
        std::vector<int> bobPath;
        traverseBob(bob, bobPath, visited);
        unlockedByBob.resize(n, std::numeric_limits<int>::max());
        for (size_t i = 0; i < bobPath.size(); ++i) {
            unlockedByBob[bobPath[i]] = i;
        }
    }

    int run() {
        traverseAlice(0, 0, -1, 0);
        return max;
    }
};

class Solution {
  public:
    static int mostProfitablePath(vector<vector<int>>& edges, int bob, vector<int>& amount) {
        Algorithm algo{bob, edges, std::move(amount)};
        return algo.run();
    }
};