#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
constexpr int N = ('z' + 1);

constexpr std::array<bool, N> getArray() {
    std::array<bool, N> ret;
    ret.fill(false);
    ret['A'] = true;
    ret['E'] = true;
    ret['I'] = true;
    ret['O'] = true;
    ret['U'] = true;
    ret['a'] = true;
    ret['e'] = true;
    ret['i'] = true;
    ret['o'] = true;
    ret['u'] = true;
    return ret;
}

class Solution {
  public:
    static constexpr bool isVowel(char c) {
        static constexpr auto arr = getArray();
        return arr[c];
    }

    static string reverseVowels(string s) {
        int n = s.size();
        int l = 0;
        int r = n - 1;
        auto inc = [&]() {
            while (l < n && !isVowel(s[l])) {
                l++;
            }
            while (r >= 0 && !isVowel(s[r])) {
                r--;
            }
        };
        inc();
        while (l < r) {
            std::swap(s[l], s[r]);
            l++;
            r--;
            inc();
        }
        return s;
    }
};