#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

static unsigned julery_isqrt(unsigned long val) {
    unsigned long temp, g = 0, b = 0x8000, bshft = 15;
    do {
        if (val >= (temp = (((g << 1) + b) << bshft--))) {
            g += b;
            val -= temp;
        }
    } while (b >>= 1);
    return g;
}

class Solution {
  public:
    static long long pickGifts(vector<int>& gifts, int k) {
        std::multiset<int> m;
        for (auto gift : gifts) {
            m.insert(gift);
        }

        for (int i = 0; i < k; i++) {
            auto it = m.end();
            it--;
            auto g = *it;
            if (g == 0) {
                break;
            }
            m.erase(it);
            m.insert(julery_isqrt(g));
        }

        return std::accumulate(m.begin(), m.end(), 0LL);
    }
};