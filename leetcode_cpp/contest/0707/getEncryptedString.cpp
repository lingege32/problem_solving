#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    string getEncryptedString(string s, int k) {
        k %= s.size();
        std::rotate(s.begin(), s.begin()+k, s.end());
        return s;
    }
};