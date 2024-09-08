#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static bool check(const vector<int>& start, int d, int mid) {
        long long prev = start[0];
        for (size_t i = 1; i < start.size(); i++) {
            long long int next = max(prev + mid, static_cast<long long>(start[i]));
            if (next > start[i] + d) {
                return false;
            }
            prev = next;
        }
        return true;
    }

    static int maxPossibleScore(vector<int>& start, int d) {
        sort(start.begin(), start.end());
        long long low = 0, hi = start.back() - start.front() + d;

        long long ans = 0;
        while (low <= hi) {
            long long mid = (low + hi) / 2;
            if (check(start, d, mid)) {
                ans = mid;
                low = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        return ans;
    }
};