#include <algorithm>
#include <vector>
#include <iostream>
using namespace std;
class Solution {
  public:
    int countDays(int days, vector<vector<int>> &meetings) {
        std::sort(meetings.begin(), meetings.end(),
                  [](const auto &lhs, const auto &rhs) {
                      auto l1 = lhs[0];
                      auto l2 = rhs[0];
                      if (l1 == l2) {
                          return lhs[1] < rhs[1];
                      }
                      return l1 < l2;
                  });
        int count = 0;
        int left = 1;
        for (const auto &meeting : meetings) {
            auto start = meeting[0];
            auto end = meeting[1];
            // std::cout<<left<<", "<<start<<", "<<end<<std::endl;
            if (start > left) {
                count += (start - left);
            }
            // std::cout<<count<<std::endl;
            left = std::max(left, end+1);
            // std::cout<<count<<std::endl;
        }
        return count+(days-left+1);
    }
};