#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * int guess(int num);
 */

int guess(int num);

class Solution {
  public:
    static int guessNumber(int n) {
        unsigned int l = 1;
        unsigned int r = n;
        r++;
        while (l < r) {
            auto m = ((r - l) / 2) + l;
            auto g = guess(m);
            if (g == -1) {
                r = m;
            } else if (g == 1) {
                l = m + 1;
            } else {
                return m;
            }
        }
        std::unreachable();
    }
};