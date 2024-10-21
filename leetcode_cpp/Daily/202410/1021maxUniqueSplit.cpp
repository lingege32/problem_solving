#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
    int maxCount = 0;
    std::unordered_set<string> seen;

  public:
    int maxUniqueSplit(const string& s) {
        backtrack(s, 0, 0);
        return maxCount;
    }

  private:
    void backtrack(const string& s, int start, int count) {
        // Prune: If the current count plus remaining characters can't exceed
        // maxCount, return
        if (count + (s.size() - start) <= static_cast<size_t>(maxCount)) {
            return;
        }

        // Base case: If we reach the end of the string, update maxCount
        if (start == static_cast<int>(s.size())) {
            maxCount = max(maxCount, count);
            return;
        }

        // Try every possible substring starting from 'start'
        for (size_t end = start + 1; end <= s.size(); ++end) {
            string substring = s.substr(start, end - start);
            // If the substring is unique
            if (seen.find(substring) == seen.end()) {
                // Add the substring to the seen set
                seen.insert(substring);
                // Recursively count unique substrings from the next position
                backtrack(s, end, count + 1);
                // Backtrack: remove the substring from the seen set
                seen.erase(substring);
            }
        }
    }
};