#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static string getHappyString(int n, int k) {
        auto group_size = (1 << (n - 1));
        std::string ret;
        if (k > group_size * 3) {
            return ret;
        }
        k--;
        ret += 'a' + (k / group_size);
        k %= group_size;
        group_size /= 2;
        auto get_c = [](char c, int k) {
            if (c == 'a') {
                return k % 2 == 0 ? 'b' : 'c';
            }
            if (c == 'b') {
                return k % 2 == 0 ? 'a' : 'c';
            }
            return k % 2 == 0 ? 'a' : 'b';
        };
        for (int i = 0; i < n - 1; ++i) {
            if (k == 0) {
                ret += get_c(ret.back(), 0);
            } else {
                ret += get_c(ret.back(), k / group_size);
            }
            k %= group_size;
            group_size /= 2;
        }

        return ret;
    }
};

/*
abab
abac
abca
abcb

*/