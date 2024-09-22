#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int countSeniors(vector<string>& details) {
        return std::count_if(details.begin(), details.end(), [](const string& detail) {
            auto ten = detail[11];
            return ten > '6' || (ten == '6' && detail[12] > '0');
        });
    }
};