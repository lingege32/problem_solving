#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArray {
    std::vector<int> dp;
    int col;

    void set(int mm, int n) {
        col = n;
        dp.resize(mm * n, -1);
    }

    int* operator[](int i) { return &dp[i * col]; }
};

class Solution {
    int s = 0;
    int mfee = 0;
    TwoDArray dp;

  public:
    int func(int idx, const vector<int>& v, bool canBuy) {
        if (idx == s) {
            return 0;
        }
        auto& dv = dp[idx][canBuy];
        if (dv != -1) {
            return dv;
        }
        if (canBuy) {
            auto take = -v[idx] + func(idx + 1, v, false);
            auto no_take = func(idx + 1, v, true);
            dv = std::max(take, no_take);
        } else {
            auto take = v[idx] - mfee + func(idx + 1, v, true);
            auto no_take = func(idx + 1, v, false);
            dv = std::max(take, no_take);
        }
        return dv;
    }

    int maxProfit(const vector<int>& v, int fee) {
        s = v.size();
        dp.set(s, 2);
        mfee = fee;
        auto ret = func(0, v, true);
        for (auto i = 0; i < s; ++i) {
            for (int j = 0; j < 2; ++j) {
                cout << "i: " << i << ", j: " << j << ", v: " << dp[i][j] << " ";
            }
        }
        return ret;
    }
};