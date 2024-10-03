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
    static int minSubarray(vector<int> &nums, int p) {
        int n = nums.size();
        long totalrem = accumulate(nums.begin(), nums.end(), 0L) % p;
        if(totalrem == 0) {
            return 0;
        }
        unordered_map<int, int> presums;
        presums[0] = -1;
        long sum = 0;
        int minlen = INT_MAX;
        for(int i = 0; i < n; i++) {
            sum += nums[i];
            long rem = sum % p; // the current remain from 0 to i
            long target = (((rem - totalrem) % p) + p) % p; // the target remain to restore.
            // if target is in presums means that 0 to presums[target] with remain target.
            // than we can remove from presums[target] to i to match p.
            if(presums.contains(target)) {
                minlen = min(i - presums[target], minlen);
            }
            presums[rem] = i;
        }
        return (minlen == INT_MAX or minlen == n) ? -1 : minlen;
    }
};