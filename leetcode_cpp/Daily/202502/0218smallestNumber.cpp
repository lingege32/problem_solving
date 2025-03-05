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
    static string smallestNumber(string pattern) {
        int n = pattern.length();
        string result;
        std::vector<int> stack(n + 1, 0);
        int index = 0;

        for (int i = 0; i <= n; i++) {
            stack[index++] = i + 1;

            if (i == n || pattern[i] == 'I') {
                while (index > 0) {
                    result += to_string(stack[--index]);
                }
            }
        }

        return result;
    }
};