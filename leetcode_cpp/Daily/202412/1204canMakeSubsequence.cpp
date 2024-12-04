#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

constexpr std::array<std::pair<char, char>, static_cast<size_t>('z') + 1> getArray() {
    std::array<std::pair<char, char>, static_cast<size_t>('z') + 1> arr;
    for (char c = 'a'; c < 'z'; ++c) {
        arr[c] = std::make_pair(c, c + 1);
    }
    arr['z'] = std::make_pair('a', 'z');
    return arr;
};

constexpr inline auto table = getArray();

class Solution {
  public:
    static bool canMakeSubsequence(const string& str1, const string& str2) {
        const size_t m = str1.size(), n = str2.size();
        if (m < n) {
            return false;
        }
        size_t idx2 = 0;
        for (auto c : str1) {
            if (idx2 == n) {
                return true;
            }
            auto c2 = str2[idx2];
            auto [n1, n2] = table[c];
            if (c2 == n1 || c2 == n2) {
                idx2++;
            }
        }
        return idx2 == str2.size();
    }
};