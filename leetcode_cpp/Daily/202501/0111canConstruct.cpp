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
    static bool canConstruct(const string& s, int k) {
        int n = s.length();
        if (n < k) {
            return false;
        }
        std::array<int, 26> count;
        count.fill(0);
        for (auto c : s) {
            count[c - 'a']++;
        }

        auto odd = std::count_if(count.begin(), count.end(), [](int x) { return x % 2 == 1; });
        return odd <= k;
    }
};