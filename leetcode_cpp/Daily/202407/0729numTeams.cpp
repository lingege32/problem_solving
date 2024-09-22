#include <bits/stdc++.h>


using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class BIT {
    vector<int> tree;

  public:
    explicit BIT(int n):
      tree(n) {};

    void update(int i, int val) {
        for (; i < static_cast<int>(tree.size()); i += (i & -i)) {
            tree[i] += val;
        }
    }

    int presum(int i) {
        int sum = 0;
        for (; i > 0; i -= (i & -i)) {
            sum += tree[i];
        }
        return sum;
    }
};

class Solution {
  public:
    static int numTeams(vector<int>& rating) {
        int n = rating.size();
        vector<pair<int, int>> v;
        v.reserve(n);
        for (int i = 0; i < n; i++) {
            v.emplace_back(rating[i], i + 1);
        }
        std::ranges::sort(v);
        BIT left(n + 1), right(n + 1);
        for (auto& [val, idx] : v) {
            right.update(idx, 1);
        }
        int ret = 0;
        for (auto& [val, idx] : v) {
            right.update(idx, -1);
            ret += left.presum(idx) * (right.presum(n) - right.presum(idx)) +
                   right.presum(idx) * (left.presum(n) - left.presum(idx));
            left.update(idx, 1);
        }
        return ret;
    }
};

class SecondSolution {
    template <typename Comp>
    static vector<vector<size_t>> createPrevTable(const vector<int>& rating) {
        std::vector<vector<size_t>> ret(rating.size(), vector<size_t>{});
        std::map<int, size_t, Comp> mapping;
        for (size_t i = 0; i < rating.size(); ++i) {
            auto r = rating[i];
            auto it = mapping.lower_bound(r);
            for (auto be = mapping.begin(); be != it; ++be) {
                ret[i].push_back(be->second);
            }
            mapping[r] = i;
        }
        return ret;
    }

    template <typename Comp>
    static int inner(const vector<int>& rating) {
        std::vector<vector<int>> dp(3, vector<int>(rating.size(), 0));
        for (auto& d : dp[0]) {
            d = 1;
        }
        auto prevTable = createPrevTable<Comp>(rating);
        for (size_t i = 1; i < 3; ++i) {
            for (size_t j = 1; j < rating.size(); ++j) {
                auto& prev = prevTable[j];
                for (auto idx : prev) {
                    dp[i][j] += dp[i - 1][idx];
                }
            }
        }
        return std::accumulate(dp[2].begin(), dp[2].end(), 0);
    }

  public:
    static int numTeams(vector<int>& rating) { return inner<std::less<>>(rating) + inner<std::greater<>>(rating); }
};

// Very slow
class FirstSolution {
    int take = 0;

    template <typename Comp>
    void count(int choose, const vector<int>& rating, int max, size_t idx, Comp&& comp) {
        if (choose == 0) {
            take++;
            return;
        }
        for (; idx < rating.size(); ++idx) {
            if (!comp(max, rating[idx])) {
                continue;
            }
            count(choose - 1, rating, rating[idx], idx + 1, std::forward<Comp>(comp));
        }
    }

  public:
    int numTeams(vector<int>& rating) {
        count(3, rating, std::numeric_limits<int>::min(), 0, std::less<>());
        count(3, rating, std::numeric_limits<int>::max(), 0, std::greater<>());
        return take;
    }
};