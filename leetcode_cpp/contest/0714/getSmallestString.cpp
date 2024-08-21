#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    string getSmallestString(string s) {
        auto len = s.size();
        for (size_t k = 1; k<len; ++k) {
            if (s[k]%2 == s[k-1]%2 && s[k-1] > s[k]) {
                std::swap(s[k], s[k-1]);
                return s;
            }
        }
        return s;
    }
};