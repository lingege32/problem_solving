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
    static int maximumSwap(int num) {
        std::string n = std::to_string(num);
        auto n2 = n;
        std::ranges::sort(n2, std::greater<>());
        int len = n.size();
        for (int i = 0; i < len; ++i) {
            if (n[i] != n2[i]) {
                for (int j = len - 1; j > i; --j) {
                    if (n2[i] == n[j]) {
                        std::swap(n[i], n[j]);
                        break;
                    }
                }
                break;
            }
        }
        return std::stoi(n);
    }
};