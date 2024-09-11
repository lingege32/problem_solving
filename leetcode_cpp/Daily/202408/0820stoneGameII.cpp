#include <bits/stdc++.h>
using namespace std;

struct TwoDimArray {
    std::vector<int> data;
    size_t c;

    void init(size_t m, size_t n) {
        data = std::vector<int>(m * n, -1);
        c = n;
    }

    int* operator[](size_t i) { return &data[i * c]; }
};

class Solution {
    TwoDimArray cache;

  public:
    int stoneGameII(vector<int>& piles) {
        for (int i = piles.size() - 2; i >= 0; --i) {
            piles[i] += piles[i + 1];
        }
        cache.init(piles.size(), piles.size());
        return play(piles, 0, 1);
    }

    int play(const vector<int>& piles_sum, size_t cur, size_t M) {
        auto maxTake = 2 * M;
        if (maxTake >= piles_sum.size() - cur) {
            return piles_sum[cur];
        }
        auto& ret = cache[cur][M];
        if (ret != -1) {
            return ret;
        }
        int minScoreFromOther = std::numeric_limits<int>::max();
        for (size_t take = 1; take <= maxTake; ++take) {
            minScoreFromOther = std::min(minScoreFromOther, play(piles_sum, cur + take, std::max(M, take)));
        }
        ret = piles_sum[cur] - minScoreFromOther;
        return ret;
    }
};
