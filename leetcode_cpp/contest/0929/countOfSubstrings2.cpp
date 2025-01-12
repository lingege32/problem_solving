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
    static bool isVowel(char ch) { return (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'); }

    static vector<int> createNextConsonant(const std::string& word) {
        int n = word.size();
        vector<int> nextConsonant(n);
        int lastConsonant = n;
        for (int i = n - 1; i >= 0; i--) {
            nextConsonant[i] = lastConsonant;
            if (!isVowel(word[i])) {
                lastConsonant = i;
            }
        }
        return nextConsonant;
    }

    static long long countOfSubstrings(const string& word, int k) {
        int n = word.size();
        unordered_map<char, int> vowels;
        int consonantCount = 0;
        long long result = 0;

        // Precompute next consonant positions
        vector<int> nextConsonant = createNextConsonant(word);
        auto eraseVowel = [&](char c) {
            if (--vowels[c] == 0) {
                vowels.erase(c);
            }
        };
        // Sliding window
        int left = 0, right = 0;
        while (right < n) {
            // Expand window
            if (isVowel(word[right])) {
                vowels[word[right]]++;
            } else {
                consonantCount++;
            }

            // Shrink window if too many consonants
            while (left <= right && consonantCount > k) {
                if (isVowel(word[left])) {
                    eraseVowel(word[left]);
                } else {
                    consonantCount--;
                }
                left++;
            }

            // Count valid substrings
            while (left < right && vowels.size() == 5 && consonantCount == k) {
                result += (nextConsonant[right] - right);
                if (isVowel(word[left])) {
                    eraseVowel(word[left]);
                } else {
                    consonantCount--;
                }
                left++;
            }

            right++;
        }

        return result;
    }
};