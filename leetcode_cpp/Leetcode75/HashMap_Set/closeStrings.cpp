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
    static bool closeStrings(const string& word1, const string& word2) {
        std::array<int, 26> f1,f2;
        f1.fill(0);
        f2.fill(0);
        for (auto c : word1) {
            f1[c - 'a']++;
        }
        for (auto c : word2) {
            f2[c - 'a']++;
        }
        for (int i = 0; i < 26; i++) {
            if ((f1[i] == 0) ^ (f2[i] == 0)) {
                return false;
            }
        }
        std::ranges::sort(f1);
        std::ranges::sort(f2);
        return f1==f2;
    }
};