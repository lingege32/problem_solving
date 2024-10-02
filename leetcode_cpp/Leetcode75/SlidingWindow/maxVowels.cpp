#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

constexpr size_t N = 'z' + 1;

constexpr std::array<char, N> getArray() {
    std::array<char, N> arr{};
    arr.fill(0);
    arr['a'] = 1;
    arr['e'] = 1;
    arr['i'] = 1;
    arr['o'] = 1;
    arr['u'] = 1;
    return arr;
}

class Solution {
  public:
    static int getValue(char c) {
        constexpr static auto arr = getArray();
        return arr[c];
    }

    static int maxVowels(const string& s, int k) {
        int total = 0;
        for (int i = 0; i < k; ++i) {
            total += getValue(s[i]);
        }
        int max = total;
        if (max == k) {
            return k;
        }
        for (size_t i = k; i < s.size(); ++i) {
            total += (getValue(s[i]) - getValue(s[i - k]));
            max = std::max(max, total);
            if (max == k) {
                return k;
            }
        }
        return max;
    }
};