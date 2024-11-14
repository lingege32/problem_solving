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
    static bool calPrime(const vector<int>& allPrimes, int& num, int threshold) {
        if (threshold >= num) {
            return false;
        }

        auto minus = num - threshold - 1;
        auto it = std::ranges::lower_bound(allPrimes | std::views::reverse, minus, std::greater<>());
        if (it != allPrimes.rend()) {
            num -= *it;
        }
        return num > threshold;
    }

    static bool primeSubOperation(vector<int>& nums) {
        std::vector<int> allPrimes{2};
        auto max = *std::ranges::max_element(nums);
        for (int i = 3; i < max; ++i) {
            auto isPrime = [&]() {
                for (auto prime : allPrimes) {
                    if (prime * prime > i) {
                        break;
                    }
                    if (i % prime == 0) {
                        return false;
                    }
                }
                return true;
            }();
            if (isPrime) {
                allPrimes.push_back(i);
            }
        }
        int prev = 0;
        for (auto& n : nums) {
            if (!calPrime(allPrimes, n, prev)) {
                return false;
            }
            prev = n;
        }

        return true;
    }
};

class OptSolution {
  public:
    static bool primeSubOperation(vector<int>& nums) {
        int len = 1005;
        vector<bool> isPrime(len, true);
        isPrime[0] = isPrime[1] = false;
        for (int i = 2; i != 33; i++) {
            if (!isPrime[i]) {
                continue;
            }
            for (int j = i; j * i < len; j++) {
                isPrime[i * j] = false;
            }
        }
        int curr = 0, next;
        for (int& num : nums) {
            if (curr >= num) {
                return false;
            }
            next = curr + 1;
            while (next < num && !isPrime[num - next]) {
                next++;
            }
            curr = next;
        }
        return true;
    }
};