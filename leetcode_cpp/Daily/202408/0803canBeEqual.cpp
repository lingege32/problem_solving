#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static bool canBeEqual(vector<int>& target, vector<int>& arr) {
        std::ranges::sort(target);
        std::ranges::sort(arr);
        return target == arr;
    }
};