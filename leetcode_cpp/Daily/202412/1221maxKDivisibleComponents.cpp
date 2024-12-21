#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

typedef long long int ll;
const int N = 3e4 + 5;

vector<vector<int>> g(N);
vector<ll> val(N);
int result;
int K;

int MaxComponents(int src, int par) {
    ll leftOver = val[src];

    for (auto i : g[src]) {
        if (i == par) {
            continue;
        }
        leftOver += MaxComponents(i, src);
    }
    if (leftOver % K == 0) {
        result++;
        leftOver = 0;
    }
    return leftOver;
}

class Solution {
  public:
    static int maxKDivisibleComponents(int n, vector<vector<int>>& edges, vector<int>& values, int k) {
        result = 0;
        K = k;
        for (int j = 0; j < n; j++) {
            g[j].clear();
            val[j] = values[j];
        }
        for (auto e : edges) {
            g[e[0]].push_back(e[1]);
            g[e[1]].push_back(e[0]);
        }

        ll leftOver = MaxComponents(0, -1);
        if (leftOver != 0) {
            result++;
        }

        return result;
    }
};