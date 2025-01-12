#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

consteval std::array<char, 501> createTable() {
    std::array<char, 501> table{};
    table[1] = 'a';
    table[2] = 'b';
    int mod = 2;
    for (int i = 3; i <= 500; i++) {
        table[i] = table[i - mod] + 1;
        if (i == mod * 2) {
            mod *= 2;
        }
    }
    return table;
}

class Solution {
  public:
    static char kthCharacter(int k) {
        static constexpr auto table = createTable();
        return table[k];
    }
};

class SecSolution {
  public:
    char recursiveKthCharacter(int k, int len, char currentChar) {
        // Base case: if k == 1, return the current character
        if (k == 1) {
            return currentChar;
        }

        // Find the length of the string after one more operation (doubling length)
        int halfLength = len / 2;

        if (k <= halfLength) {
            // k is in the first half of the string, so we recurse into that half
            return recursiveKthCharacter(k, halfLength, currentChar);
        }
        // k is in the second half, so we need to shift the characters
        // Recurse with k relative to the second half
        char nextChar = (currentChar == 'z') ? 'a' : currentChar + 1;
        return recursiveKthCharacter(k - halfLength, halfLength, nextChar);
    }

    char kthCharacter(int k) {
        int len = 1;  // Start with word = "a", which has length 1
        char currentChar = 'a';

        // Find the smallest length where the length is greater than or equal to k
        while (len < k) {
            len *= 2;
        }

        // Start the recursive search for the k-th character
        return recursiveKthCharacter(k, len, currentChar);
    }
};