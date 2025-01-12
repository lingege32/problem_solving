#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct Key {
    long long consume = 0;
    long long next = 0;
    int base = 0;

    bool operator<(const Key& other) const {
        if (consume != other.consume) {
            return consume > other.consume;
        }
        return base > other.base;
    }
};

class Solution {
  public:
    static long long minNumberOfSeconds(int total, vector<int>& workerTimes) {
        // binary search!
        
        std::ranges::sort(workerTimes, std::greater<>());
        auto getTime = [&](int t, long long max_time) {
            // t * (res * (res + 1) / 2) <= max_time
            return (-1 + sqrt(1 + (8 * max_time / t))) / 2;
        };
        // available is to check if the max_time for every workertime can finish the mountain.
        auto available = [&](long long max_time) {
            long long s = 0;
            for (const auto& wt : workerTimes) {
                s += getTime(wt, max_time);
                if (s >= total) {
                    return true;
                }
            }
            return false;
        };
        long long M = 1'000'000LL * (100000LL * 100001LL) / 2;
        long long l = 0, r = M, res = M;
        while (l <= r) {
            long long mid = (l + r) / 2;
            if (available(mid)) {
                res = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return res;
    }
};