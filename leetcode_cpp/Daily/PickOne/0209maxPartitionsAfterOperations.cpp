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
    static int count(int n) { return __builtin_popcount(n); }

    static int maxPartitionsAfterOperations(string s, int k) {
        if (k == 26) {
            return 1;
        }
        int n = s.size();
        s = "@" + s;
        s += "@";
        std::vector<int> pref(n + 2), pbits(n + 2);
        int pbit = 0;
        int prefix = 0;
        for (int i = 1; i <= n; ++i) {
            auto c = s[i];
            auto bit = 1 << (c - 'a');
            pbit |= bit;
            if (count(pbit) > k) {
                prefix++;
                pbit = bit;
            }
            pref[i] = prefix;
            pbits[i] = pbit;
        }
        if (pref.back() == 1 && count(pbits.back()) < k) {
            return 1;
        }

        std::vector<int> suff(n + 2), sbits(n + 2);
        int sbit = 0;
        int suffix = 0;
        for (int i = n; i >= 1; --i) {
            auto c = s[i];
            auto bit = 1 << (c - 'a');
            sbit |= bit;
            if (count(sbit) > k) {
                suffix++;
                sbit = bit;
            }
            suff[i] = suffix;
            sbits[i] = sbit;
        }

        int ret = 0;
        for (int i = 1; i <= n; ++i) {
            int val = pref[i - 1] + suff[i + 1];
            const auto p = pbits[i - 1];
            const auto sc = sbits[i + 1];
            const auto pcnt = count(p);
            const auto scnt = count(sc);
            const auto cnt = count(p | sc);
            if (cnt < k) {
                val += 1;
            } else if (pcnt == k && scnt == k && cnt < 26) {
                val += 3;
            } else {
                val += 2;
            }
            ret = std::max(val, ret);
        }

        return ret;
    }
};