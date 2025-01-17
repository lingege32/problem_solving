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
    static bool doesValidArrayExist(const vector<int>& derived) {
        return !std::accumulate(derived.begin(), derived.end(), 0, std::bit_xor<>());
    }
};