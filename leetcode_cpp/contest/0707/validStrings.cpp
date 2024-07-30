#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    vector<string> validStrings(int n) {
        return inner(n);
    }
    vector<string> inner(int n) {
        if (n == 1) {
            return {"0", "1"};
        }
        auto less = inner(n - 1);
        std::vector<string> another;
        for (const auto &v : less) {
            if (v.back() == '1') {
                another.push_back(v + "0");
            }
        }
        for (auto &v : less) {
            v += "1";
        }
        less.insert(less.end(), another.begin(), another.end());
        return less;
    }
};