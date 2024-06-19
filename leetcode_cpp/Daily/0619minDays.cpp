#include <algorithm>
#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    int minDays(vector<int> &bloomDay, int m, int k) {
        if (static_cast<size_t>(m)*k > bloomDay.size()) {
            return -1;
        }
        auto right = 1 + *std::max_element(bloomDay.begin(), bloomDay.end());
        auto left = 0;
        while (left < right) {
            auto mid = left + (right - left) / 2;
            int con=0;
            int buk=0;
            for(auto b: bloomDay) {
                if (b <= mid) {
                    con++;
                    if (con == k) {
                        con=0;
                        buk++;
                    }
                } else {
                    con=0;
                }
                if (buk==m) {
                    break;
                }
            }
            if (buk == m) {
                right=mid;
            } else {
                left=mid+1;
            }
        }
        return left;
    }
};