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
        static int numOfSubarrays(vector<int>& arr) {
            long long oddCount = 0, prefixSum = 0;
            for(int a : arr) {
                prefixSum += a;
                oddCount += prefixSum % 2;
            }
            oddCount += (arr.size() - oddCount) * oddCount;
            return oddCount % 1'000'000'007;
        }
    };