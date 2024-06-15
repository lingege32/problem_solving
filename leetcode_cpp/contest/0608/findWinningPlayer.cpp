#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
  public:
    int findWinningPlayer(vector<int> &skills, int k) {
        if (k == 1) {
            return skills[0] > skills[1] ? 0 : 1;
        }
        std::vector<size_t> table;
        int max = -1;
        for (size_t idx = 0; idx < skills.size(); ++idx) {
            if (skills[idx] > max) {
                max = skills[idx];
                table.push_back(idx);
            }
        }
        // for (auto t : table) {
        //     std::cout << t << " ";
        // }
        // std::cout << "\n";
        if (table.size() > 1 && table[1] > static_cast<size_t>(k)) {
            return 0;
        }
        for (size_t idx = 2U; idx < table.size(); ++idx) {
            if (table[idx] - table[idx - 1] >= static_cast<size_t>(k)) {
                return table[idx - 1];
            }
        }

        return table.back();
    }
};