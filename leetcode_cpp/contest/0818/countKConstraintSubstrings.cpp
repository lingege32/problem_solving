#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int countKConstraintSubstrings(string s, int k) {
        auto len = s.size();
        int ret = 0;
        auto check = [k, &ret](int o, int z) {
            if (o <= k || z <= k) {
                ret++;
            }
        };
        for (size_t i = 1; i <= len; ++i) {
            int o = 0;
            for (size_t j = 0; j < i; ++j) {
                if (s[j] == '1') {
                    o++;
                }
            }
            size_t be = 0;
            size_t en = i - 1;
            check(o, i - o);
            for (en = i; en < len; ++en, ++be) {
                if (s[en] == '1') {
                    o++;
                }
                if (s[be] == '1') {
                    o--;
                }
                check(o, i - o);
            }
            // std::cout<<"i: "<<i<<", ret: "<<ret<<std::endl;
        }
        return ret;
    }
};