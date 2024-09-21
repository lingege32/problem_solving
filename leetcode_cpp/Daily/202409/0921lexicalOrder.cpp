#include <bits/stdc++.h>
using namespace std;

class Solution {
    int limit = 0;
    std::vector<int> ret;

  public:
    vector<int> lexicalOrder(int n) {
        limit = n;
        ret.reserve(n);
        for (int i = 1; i <= 9; ++i) {
            add(i);
        }
        return std::move(ret);
    }

    void add(int n) {
        if (n > limit) {
            return;
        }
        ret.push_back(n);
        n *= 10;
        for (int i = 0; i <= 9; ++i) {
            add(n + i);
        }
    }
};

class OptimzeSolution {
  public:
    static vector<int> lexicalOrder(int n) {
        vector<int> lexicographicalNumbers;
        int currentNumber = 1;

        // Generate numbers from 1 to n
        for (int i = 0; i < n; ++i) {
            lexicographicalNumbers.push_back(currentNumber);

            // If multiplying the current number by 10 is within the limit, do
            // it
            if (currentNumber * 10 <= n) {
                currentNumber *= 10;
            } else {
                // Adjust the current number by moving up one digit
                while (currentNumber % 10 == 9 || currentNumber >= n) {
                    currentNumber /= 10;  // Remove the last digit
                }
                currentNumber += 1;  // Increment the number
            }
        }

        return lexicographicalNumbers;
    }
};