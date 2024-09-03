#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    long long maxEnergyBoost(vector<int>& energyDrinkA, vector<int>& energyDrinkB) {
        long long prev_a = energyDrinkA[0];
        long long prev_b = energyDrinkB[0];
        for (size_t i = 1; i < energyDrinkA.size(); ++i) {
            long long cur_a = std::max(prev_b, prev_a + energyDrinkA[i]);
            long long cur_b = std::max(prev_a, prev_b + energyDrinkB[i]);
            prev_a = cur_a;
            prev_b = cur_b;
        }
        return std::max(prev_a, prev_b);
    }
};