#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

constexpr int MOD = 1e9 + 7;

std::array<int, 100027> createTable() {
    std::array<int, 100027> table;
    table.fill(0);
    for (int i = 0; i < 26; ++i) {
        table[i] = 1;
    }
    return table;
}

int lookup(int v) {
    static auto table = createTable();
    auto& ret = table[v];
    if (ret != 0) {
        return ret;
    }
    ret = (lookup(v - 26) + lookup(v - 25)) % MOD;
    return ret;
}

class Solution {
  public:
    static int lengthAfterTransformations(const string& s, int t) {
        std::unordered_map<int, int> table;
        for (int i = 0; i < 26; ++i) {
            table[i] = 1;
        }
        int ret = 0;
        for (auto c : s) {
            int v = t + (c - 'a');
            ret += lookup(v);
            ret %= MOD;
        }
        return ret;
    }
};