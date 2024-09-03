#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    vector<long long> countKConstraintSubstrings(string s, int k, vector<vector<int>>& queries) {
        int n = s.size();
        vector<long long> ans, pre(n), prev(n);
        int l = 0, zeros = 0, ones = 0;
        for (int r = 0; r < n; r++) {
            if (s[r] == '1') {
                ones++;
            } else {
                zeros++;
            }
            while (zeros > k && ones > k) {
                if (s[l] == '1') {
                    ones--;
                } else {
                    zeros--;
                }
                l++;
            }
            prev[r] = l;
        }

        // <!-- Calculating prefix sums -->
        pre[0] = 1;
        for (int i = 1; i < n; i++) {
            pre[i] = pre[i - 1] + i - prev[i] + 1;
        }
        for (auto& q : queries) {
            long long l = q[0];
            long long r = q[1];
            long long cur = 0;
            long long pos = r + 1;
            // <!-- Finding pos -->
            while (l <= r) {
                long long mid = (l + r) / 2;
                if (prev[mid] >= q[0]) {
                    pos = mid;
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }

            // <!-- If there exists such index -->

            if (pos <= q[1]) {
                cur += pre[q[1]];
                if (pos > 0) {
                    cur -= pre[pos - 1];
                }
            }
            cur += (pos - q[0]) * (pos - q[0] + 1) / 2;

            ans.push_back(cur);
        }
        return ans;
    }
};

class MySolution_Slow {
  public:
    vector<long long> countKConstraintSubstrings(string s, int k, vector<vector<int>>& queries) {
        auto len = s.size();
        vector<long long> dp(len, 0);
        dp[0] = s[0] == '1' ? 1 : 0;
        for (size_t i = 1; i < len; ++i) {
            dp[i] = dp[i - 1] + (s[i] == '1' ? 1 : 0);
        }
        std::unordered_map<int, std::unordered_map<int, long long>> cache;
        std::function<long long(int, int)> find = [&](int be, int en) {
            if (be == en) {
                return 1LL;
            }
            auto [it, inserted] = cache[be].emplace(en, 0);
            if (!inserted) {
                return it->second;
            }

            auto prev = find(be + 1, en);
            int o = 0;
            int l = 1;
            // std::cout<<"be: "<<be<<", en: "<<en<<std::endl;
            for (auto i = be; i <= en; ++i, ++l) {
                if (s[i] == '1') {
                    o++;
                }
                // std::cout<<"i: "<<i<<", o: "<<o<<", l: "<<l<<", k: "<<k<<", prev: "<<prev<<std::endl;
                if (o <= k || (l - o) <= k) {
                    prev++;
                } else {
                    break;
                }
            }
            it->second = prev;
            return prev;
        };
        vector<long long> ret;
        ret.reserve(queries.size());
        for (const auto& query : queries) {
            auto be = query[0];
            auto en = query[1];
            ret.push_back(find(be, en));
        }
        return ret;
    }
};