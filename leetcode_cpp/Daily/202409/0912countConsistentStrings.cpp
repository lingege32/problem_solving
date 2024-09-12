#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int countConsistentStrings(const string& allowed, vector<string>& words) {
        std::array<char, 26> allowedChars;
        allowedChars.fill(0);
        for (auto c : allowed) {
            allowedChars[c - 'a'] = 1;
        }
        int a = 0;
        for (const auto& word : words) {
            bool flag = true;
            for (auto c : word) {
                if (allowedChars[c - 'a'] == 0) {
                    flag = false;
                    break;
                }
            }
            if (flag) {
                a++;
            }
        }
        return a;
    }
};