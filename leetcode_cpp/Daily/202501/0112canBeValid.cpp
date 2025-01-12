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
    static bool canBeValid(const string& parentheses, const string& lockedStatus) {
        int stringLength = parentheses.size();
        if (stringLength % 2 == 1) {
            return false;
        }

        stack<int> openIndices;
        stack<int> unlockedIndices;

        for (int i = 0; i < stringLength; i++) {
            if (lockedStatus[i] == '0') {
                unlockedIndices.push(i);
            } else if (parentheses[i] == '(') {
                openIndices.push(i);
            } else if (parentheses[i] == ')') {
                if (!openIndices.empty()) {
                    openIndices.pop();
                } else if (!unlockedIndices.empty()) {
                    unlockedIndices.pop();
                } else {
                    return false;
                }
            }
        }

        while (!openIndices.empty() && !unlockedIndices.empty() && openIndices.top() < unlockedIndices.top()) {
            openIndices.pop();
            unlockedIndices.pop();
        }

        if (openIndices.empty() && !unlockedIndices.empty()) {
            return openIndices.size() % 2 == 0;
        }
        return openIndices.empty();
    }
};