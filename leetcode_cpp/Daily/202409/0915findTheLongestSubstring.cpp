#include <bits/stdc++.h>
using namespace std;

constexpr std::array<char, 26> getVowels() {
    std::array<char, 26> vowels;
    vowels.fill(0);
    vowels['a' - 'a'] = 1;
    vowels['e' - 'a'] = 2;
    vowels['i' - 'a'] = 4;
    vowels['o' - 'a'] = 8;
    vowels['u' - 'a'] = 16;
    return vowels;
};

class Solution {
  public:
    static void updateMask(char& mask, char c) {
        constexpr static std::array<char, 26> masks = getVowels();
        mask ^= masks[c - 'a'];
    }

    static int findTheLongestSubstring(string s) {
        std::array<int, 32> prev;
        prev.fill(-1);
        prev[0] = 0;
        char pre_mask = 0;
        int ret = 0;
        for (int idx = 0; idx < static_cast<int>(s.length()); ++idx) {
            updateMask(pre_mask, s[idx]);
            s[idx] = pre_mask;
            if (prev[pre_mask] == -1) {
                prev[pre_mask] = idx + 1;
            }
            ret = std::max(ret, idx - prev[pre_mask] + 1);
        }
        return ret;
    }
};