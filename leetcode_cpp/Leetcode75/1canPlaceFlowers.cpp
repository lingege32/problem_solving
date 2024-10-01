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
    static bool canPlaceFlowers(vector<int>& flowerbed, int n) {
        flowerbed.push_back(0);
        flowerbed.push_back(1);
        int k = flowerbed.size();
        int left = -2;
        int right = std::distance(flowerbed.begin(), std::ranges::find(flowerbed, 1));
        for (; left != k;) {
            int dis = right - left - 1;
            if (dis >= 3) {
                dis -= 2;
                int canPlace = (dis / 2) + (dis % 2);
                n -= canPlace;
                if (n <= 0) {
                    return true;
                }
            }

            left = right;
            right = std::distance(flowerbed.begin(), std::find(flowerbed.begin() + right + 1, flowerbed.end(), 1));
        }

        return n<=0;
    }
};