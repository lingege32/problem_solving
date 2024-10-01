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
    static string gcdOfStrings(const string& str1, const string& str2) {
        // If concatenating both strings in different orders produces the same result,
        // then there exists a valid GCD string
        if (str1 + str2 != str2 + str1) {
            return "";  // No common divisor string possible
        }

        // Compute the greatest common divisor of the lengths of the two strings
        int gcdLength = gcd((int)str1.size(), (int)str2.size());

        // The GCD string is the first 'gcdLength' characters of either string
        return str1.substr(0, gcdLength);
    }
};