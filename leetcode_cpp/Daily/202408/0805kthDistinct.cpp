#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static string kthDistinct(vector<string>& arr, int k) {
        std::unordered_map<std::string, int> count;
        for (const auto& s : arr) {
            count[s]++;
        }
        for (const auto& s : arr) {
            if (count[s] == 1) {
                if (--k == 0) {
                    return s;
                }
            }
        }
        return "";
    }
};