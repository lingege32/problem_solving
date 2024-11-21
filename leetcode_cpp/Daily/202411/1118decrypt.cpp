#include <bits/stdc++.h>

#include <algorithm>
#include <ranges>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static vector<int> decrypt(vector<int>& code, int k) {
        int n = code.size();
        std::vector<int> ret(n, 0);
        if (k == 0) {
            return ret;
        }
        if (k > 0) {
            for (int i = 0; i < k; ++i) {
                code.push_back(code[i]);
            }
        } else {
            k = -k + 1;
            for (int i = n - k; i < n; ++i) {
                code.push_back(code[i]);
            }
            std::ranges::rotate(std::ranges::reverse_view(code), code.rbegin() + k);
            k--;
        }
        std::inclusive_scan(code.begin(), code.end(), code.begin());
        for (int i = 0; i < n; ++i) {
            ret[i] = code[i + k] - code[i];
        }
        return ret;
    }
};