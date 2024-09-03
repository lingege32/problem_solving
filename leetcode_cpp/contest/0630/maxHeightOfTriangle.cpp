#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int maxHeightOfTriangle(int red, int blue) {
        int level = 1;
        int left = 0, right = 0;
        bool isLeft = true;
        auto check = [&]() {
            return (red >= left && blue >= right) ||
                   (red >= right && blue >= left);
        };
        for(;;){
            if (isLeft) {
                left += level;
            } else {
                right += level;
            }
            if (!check()) {
                break;
            }
            isLeft = !isLeft;
            level++;
        }
        return level - 1;
    }
};