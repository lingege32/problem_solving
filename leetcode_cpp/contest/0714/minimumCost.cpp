#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    int minimumCost(int m, int n, vector<int>& horizontalCut, vector<int>& verticalCut) {
        int h = 1;
        int v = 1;
        int ret = 0;
        static int dummy = 0;
        while (h < m || v < n) {
            int& max_h = h==m? dummy : *max_element(horizontalCut.begin(), horizontalCut.end());
            int& max_v = v==n? dummy : *max_element(verticalCut.begin(), verticalCut.end());
            if (max_h > max_v) {
                ret += max_h * v;
                max_h = 0;
                h+=1;
            } else {
                ret+=max_v * h;
                max_v=0;
                v+=1;
            }
        }
        return ret;
    }
};