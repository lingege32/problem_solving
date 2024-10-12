#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int minGroups(vector<vector<int>>& intervals) {
        vector<int> A, B;
        for (auto& i : intervals) {
            A.push_back(i[0]), B.push_back(i[1]);
        }
        std::ranges::sort(A);
        std::ranges::sort(B);
        int l = 0, r = 0, cnt = 0, ans = 0, n = A.size();
        while (l < n) {
            if (A[l] <= B[r]) {
                cnt++, ans = max(ans, cnt), l++;
            } else {
                cnt--, r++;
            }
        }
        return ans;
    }
};