#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static string convertDateToBinary(string date) {
        std::array<int, 3> dates;
        auto pos = date.find('-');
        size_t idx = 0;
        while (pos != std::string::npos) {
            dates[idx++] = std::stoi(date.substr(0, pos));
            date = date.substr(pos + 1);
            pos = date.find('-');
        }
        dates[idx++] = std::stoi(date.substr(0, pos));
        std::string ans;
        ans += toBit(dates[0]);
        ans += '-';
        ans += toBit(dates[1]);
        ans += '-';
        ans += toBit(dates[2]);
        return ans;
    }

    static std::string toBit(int num) {
        std::string ans;
        while (num) {
            ans += std::to_string(num % 2);
            num /= 2;
        }
        std::reverse(ans.begin(), ans.end());
        return ans;
    }
};