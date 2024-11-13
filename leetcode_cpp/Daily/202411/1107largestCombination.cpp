#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
template<int N>
constexpr int getBits() {
    if constexpr (N > 0) {
        constexpr int Next = N>>1;
        return 1 + getBits<Next>();
    } else {
        return 0;
    }
}
class Solution {
  public:
    static int largestCombination(vector<int>& candidates) {
        std::array<int, 32> cnt;
        cnt.fill(0);
        for (auto candidate : candidates) {
            for (int i = 0; candidate > 0 && i < getBits<100000>(); i++) {
                cnt[i] += candidate & 1;
                candidate >>= 1;
            }
        }
        return *std::ranges::max_element(cnt);
    }
};