#include "header1.h"

int Solution::maxDistToClosest(vector<int> &seats) {
    int left_sitting = 0;
    for (; seats[left_sitting] == 0; ++left_sitting) {
    }
    int max_distance = left_sitting;
    int ss = seats.size();
    for (int right_sitting = left_sitting + 1; right_sitting < ss;
         ++right_sitting) {
        if (seats[right_sitting] == 1) {
            max_distance =
                std::max(max_distance, (right_sitting - left_sitting) / 2);
            left_sitting = right_sitting;
        }
    }
    for (int i = ss - 1; i >= 0; --i) {
        if (seats[i] == 1) {
            max_distance = std::max(max_distance, ss - 1 - i);
            break;
        }
    }

    return max_distance;
}