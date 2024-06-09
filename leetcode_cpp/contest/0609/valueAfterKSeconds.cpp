#include <vector>
class Solution {
  public:
    void push_prime(std::vector<int> &v, int n) {
        if (n == 1) {
            return;
        }
        for (int i = 2; i <= n / 2; ++i) {
            if (n % i == 0) {
                v.push_back(i);
                push_prime(v, n / i);
                return;
            }
        }
        v.push_back(n);
    }
    int cmn(int m, int n) {
        if (m / 2 < n) {
            return cmn(m, m - n);
        }
        std::vector<int> top, down;
        for (int k = 0; k < n; ++k) {
            top.push_back(m - k);
        }

        for (int k = 2; k <= n; k++) {
            push_prime(down,k);
        }
        for (auto d : down) {
            for (auto &t : top) {
                if (t % d == 0) {
                    t /= d;
                    break;
                }
            }
        }
        unsigned long a = 1;
        for (auto t : top) {
            a *= t;
            a %= 1000000007;
        }
        return a;
    }
    int valueAfterKSeconds(int n, int k) {
        // pascal triangle
        //
        return cmn(n + k - 1, n - 1);
    }
};