#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int getLucky(string s, int k) {
        k-=1;
        int ret = 0;
        for(auto c : s) {
            int v = c - 'a' + 1;
            while(v) {
                ret += v % 10;
                v /= 10;
            }
        }      
        while (k--) {
            int tmp = 0;
            while (ret) {
                tmp += ret % 10;
                ret /= 10;
            }
            std::swap(tmp, ret);
        }
        return ret;
    }
};