#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

constexpr size_t N = 26;
using Counter = std::array<int, N>;

class Solution {
  public:
    static bool checkInclusion(const string& s1, const string& s2) {
        if (s1.size() > s2.size()) {
            return false;
        }

        int n = s1.size();
        Counter s1Counter;
        s1Counter.fill(0);
        for (auto c : s1) {
            s1Counter[c - 'a']++;
        };
        for (int i = 0; i < n; ++i) {
            s1Counter[s2[i] - 'a']--;
        }
        if (std::ranges::all_of(s1Counter, [](int x) { return x == 0; })) {
            return true;
        }
        for (size_t i = n; i < s2.size(); ++i) {
            s1Counter[s2[i] - 'a']--;
            s1Counter[s2[i - n] - 'a']++;
            if (std::ranges::all_of(s1Counter, [](int x) { return x == 0; })) {
                return true;
            }
        }
        return false;
    }
};