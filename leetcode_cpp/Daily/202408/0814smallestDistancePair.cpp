#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int smallestDistancePair(std::vector<int>& numbers, int k) {
        std::sort(numbers.begin(), numbers.end());
        int minDistance = 0, maxDistance = numbers.back() - numbers.front();

        while (minDistance < maxDistance) {
            int midDistance = minDistance + (maxDistance - minDistance) / 2;
            if (countPairsWithinDistance(numbers, midDistance) < k) {
                minDistance = midDistance + 1;
            } else {
                maxDistance = midDistance;
            }
        }

        return minDistance;
    }

  private:
    static int countPairsWithinDistance(const std::vector<int>& numbers, int targetDistance) {
        int count = 0, left = 0;
        for (size_t right = 1; right < numbers.size(); ++right) {
            while (numbers[right] - numbers[left] > targetDistance) {
                ++left;
            }
            count += right - left;
        }
        return count;
    }
};
