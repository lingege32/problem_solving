#include <bits/stdc++.h>
using namespace std;

auto _ = []() {
    ios::sync_with_stdio(false);
    cin.tie();
    cout.tie();
    return 0;
}();

class Solution {
  public:
    std::array<short, 1005> G;
    short ans;

    short find(short x) {
        while (G[x] != x) {
            G[x] = G[G[x]];  // Path compression
            x = G[x];
        }
        return G[x];
    }

    void unite(short x, short y) {
        x = find(x);
        y = find(y);
        if (x != y) {
            ans++;
            G[y] = x;  // Union operation
        }
    }

    int removeStones(vector<vector<int>>& stones) {
        ans = 0;
        for (size_t i = 1; i <= stones.size(); i++) {
            G[i] = i;  // Initialize Union-Find
        }

        std::array<short, 10001> mp;
        for (size_t i = 0; i < stones.size(); i++) {
            if (mp[stones[i][0]]) {
                unite(i + 1, mp[stones[i][0]]);
            } else {
                mp[stones[i][0]] = i + 1;
            }
        }

        mp.fill(0);
        for (size_t i = 0; i < stones.size(); i++) {
            if (mp[stones[i][1]]) {
                unite(i + 1, mp[stones[i][1]]);
            } else {
                mp[stones[i][1]] = i + 1;
            }
        }

        return ans;
    }
};