#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    long long minimumCost(int m, int n, vector<int>& horizontalCut, vector<int>& verticalCut) {
        int h = 1;
        int v = 1;
        long long ret = 0;
        std::priority_queue<int> hp (horizontalCut.begin(), horizontalCut.end());
        std::priority_queue<int> vp (verticalCut.begin(), verticalCut.end());
        while (!hp.empty() || !vp.empty()) {
            int max_h = hp.empty()? 0 : hp.top();
            int max_v = vp.empty()? 0 : vp.top();
            if (max_h > max_v) {
                ret += max_h * v;
                hp.pop();
                h+=1;
            } else {
                ret+=max_v * h;
                vp.pop();
                v+=1;
            }
        }
        return ret;
    }
};