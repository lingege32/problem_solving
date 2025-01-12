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
    static char recursive(long long base2, long long k, vector<int>& reverseOperations, char curCh) {
        if (k == 1) {
            return curCh;
        }
        auto half = base2 / 2;
        // std::cout<<"base2: "<<base2<<" k: "<<k<<" half: "<<half<<" curCh: "<<curCh<<std::endl;
        auto op = reverseOperations.back();
        reverseOperations.pop_back();
        if (k > half) {
            k -= half;
            if (op == 1) {
                curCh = (curCh == 'z') ? 'a' : curCh + 1;
            }
        }
        return recursive(half, k, reverseOperations, curCh);
    }

    static char kthCharacter(long long k, vector<int>& operations) {
        long long n = 1;
        size_t nPower = 0;
        while (n < k) {
            n *= 2;
            nPower++;
        }
        operations.resize(nPower);
        // std::ranges::reverse(operations);
        return recursive(n, k, operations, 'a');
    }
};