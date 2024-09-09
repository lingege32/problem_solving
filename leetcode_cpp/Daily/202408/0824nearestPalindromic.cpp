#include <bits/stdc++.h>
using std::string;

using namespace std;

class Solution {
  public:
    static string nearestPalindromic(const string& numberStr) {
        long long number = stoll(numberStr);

        // Edge cases for small numbers
        if (number <= 10) {
            return to_string(number - 1);
        }
        if (number == 11) {
            return "9";
        }

        // Special case for 18-digit number with all 9s, thanks to dcodeDV for pointing this out
        if (numberStr == "999999999999999999") {
            return "1000000000000000001";
        }

        int length = numberStr.length();
        long long leftHalf = stoll(numberStr.substr(0, (length + 1) / 2));

        vector<long long> palindromeCandidates(5);
        palindromeCandidates[0] = generatePalindromeFromLeft(leftHalf - 1, length % 2 == 0);
        palindromeCandidates[1] = generatePalindromeFromLeft(leftHalf, length % 2 == 0);

        // Handle potential overflow for leftHalf + 1
        if (leftHalf < 999999999) {
            palindromeCandidates[2] = generatePalindromeFromLeft(leftHalf + 1, length % 2 == 0);
        } else {
            palindromeCandidates[2] = stoll("1" + string(length - 1, '0') + "1");
        }
        // std::cout<<"length: "<<length<<std::endl;
        palindromeCandidates[3] = static_cast<long long>(pow(10LL, length - 1)) - 1;
        palindromeCandidates[4] = static_cast<long long>(pow(10LL, length)) + 1;

        long long nearestPalindrome = LLONG_MAX;
        long long minDifference = LLONG_MAX;
        std::sort(palindromeCandidates.begin(), palindromeCandidates.end());
        for (long long candidate : palindromeCandidates) {
            if (candidate == number) {
                continue;
            }
            // std::cout<<candidate<<std::endl;
            long long difference = abs(candidate - number);
            if (difference < minDifference || (difference == minDifference && candidate < nearestPalindrome)) {
                minDifference = difference;
                nearestPalindrome = candidate;
            }
        }

        return to_string(nearestPalindrome);
    }

  private:
    static long long generatePalindromeFromLeft(long long leftHalf, bool isEvenLength) {
        long long palindrome = leftHalf;
        if (!isEvenLength) {
            leftHalf /= 10;
        }
        while (leftHalf > 0) {
            palindrome = palindrome * 10 + leftHalf % 10;
            leftHalf /= 10;
        }
        return palindrome;
    }
};

// https://leetcode.com/problems/find-the-closest-palindrome/submissions/1366449944/