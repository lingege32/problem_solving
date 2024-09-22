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

#define ll int

// My Segment Tree Template (inspired from Pashka)
class item {
  public:
    ll val;

    item() { val = 0; }

    explicit item(ll v) { val = v; }
};

class SegTree {
  public:
    ll size;
    vector<item> values;

    item NEUTRAL_ELEMENT;

    static item merge(item l, item r) { return item(l.val + r.val); }

    static item single(ll v) { return item(v); }

    void init(ll n) {
        size = 1;
        while (size < n) {
            size *= 2;  // for last level array with some extra nodes for complete binary tree
        }
        values.resize((2 * size) - 1);  // for complete binary tree
    }

    void build(vector<int>& a, ll x, ll lx, ll rx) {
        // base case
        if (rx - lx == 1) {
            if (lx < int(a.size())) {
                values[x] = single(a[lx]);
            }
            return;
        }

        ll m = (lx + rx) / 2;
        build(a, (2 * x) + 1, lx, m);
        build(a, (2 * x) + 2, m, rx);
        values[x] = merge(values[(2 * x) + 1], values[(2 * x) + 2]);
    }

    void build(vector<int>& a) { build(a, 0, 0, size); }

    void set(ll i, ll v, ll x, ll lx, ll rx) {
        // base case
        if (rx - lx == 1) {
            values[x] = single(v);
            return;
        }

        ll m = (lx + rx) / 2;
        if (i < m) {
            set(i, v, (2 * x) + 1, lx, m);
        } else {
            set(i, v, (2 * x) + 2, m, rx);
        }
        values[x] = merge(values[(2 * x) + 1], values[(2 * x) + 2]);
    }

    void set(ll i, ll v) { set(i, v, 0, 0, size); }

    item calc(ll l, ll r, ll x, ll lx, ll rx) {
        // base cases
        if (lx >= r || l >= rx) {
            return NEUTRAL_ELEMENT;
        }
        if (lx >= l && rx <= r) {
            return values[x];
        }

        ll m = (lx + rx) / 2;
        item s1 = calc(l, r, (2 * x) + 1, lx, m);
        item s2 = calc(l, r, (2 * x) + 2, m, rx);
        return merge(s1, s2);
    }

    item query(ll l, ll r) { return calc(l, r + 1, 0, 0, size); }
};

class SegmentTreeSolution {
  public:
    static int numTeams(vector<int>& rating) {
        int mx = 1e5;
        SegTree st1, st2;
        st1.init(mx), st2.init(mx);

        int n = rating.size();
        for (int i = 0; i < n; ++i) {
            st2.set(rating[i], 1);
        }

        int res = 0;
        for (int i = 0; i < n; ++i) {
            int lefti = st1.query(0, rating[i] - 1).val;
            int leftd = st1.query(rating[i] + 1, mx).val;
            int righti = st2.query(rating[i] + 1, mx).val - leftd;
            int rightd = st2.query(0, rating[i] - 1).val - lefti;
            st1.set(rating[i], 1);
            res += lefti * righti + leftd * rightd;
        }
        return res;
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