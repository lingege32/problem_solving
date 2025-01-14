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
    static vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
        std::vector<int> ret(A.size());
        int n = A.size();
        std::vector<uint8_t> freq(A.size() + 1, 0);
        int cnt = 0;
        for (int i = 0; i < n; ++i) {
            if (freq[A[i]]++ == 1) {
                cnt++;
            }
            if (freq[B[i]]++ == 1) {
                cnt++;
            }
            ret[i] = cnt;
        }
        return ret;
    }
};