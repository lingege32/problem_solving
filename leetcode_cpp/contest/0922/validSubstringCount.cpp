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
    static long long validSubstringCount(const string& word1, const string& word2) {
        // two pointer
        std::array<int, 26> count;
        count.fill(0);
        for (auto c : word2) {
            count[c - 'a']++;
        }
        int n = word2.size();
        int left = 0;
        int n1 = word1.size();
        long long ret = 0;
        for (int right = 0; right < n1; ++right) {
            auto c = word1[right];
            auto& cnt = count[c - 'a'];
            cnt--;
            if (cnt >= 0) {
                n--;
            }
            while (n == 0) {
                auto leftc = word1[left++];
                if (0 == count[leftc - 'a']++) {
                    n++;
                }
            }
            ret += left;
        }
        return ret;
    }
};