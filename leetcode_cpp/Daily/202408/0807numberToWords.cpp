#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    constexpr static std::array<std::string, 20> MAPPING = {
        "",    "One",    "Two",    "Three",    "Four",     "Five",    "Six",     "Seven",     "Eight",    "Nine",
        "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"};

    constexpr static std::array<std::string, 10> TENS = {"",      "",      "Twenty",  "Thirty", "Forty",
                                                         "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"};

    static std::string join(const std::vector<std::string>& vec) {
        std::string ret;
        ret += vec[0];
        for (size_t i = 1; i < vec.size(); i++) {
            ret += " ";
            ret += vec[i];
        }
        return ret;
    }

    static string nToWord(int num) {
        if (num == 0) {
            return "";
        }
        // handle the number < 1000
        std::vector<std::string> retarray;
        if (num >= 100) {
            retarray.push_back(MAPPING[num / 100]);
            retarray.emplace_back("Hundred");
        }
        int r = num % 100;
        if (r > 0) {
            if (r < 20) {
                retarray.push_back(MAPPING[r]);
            } else {
                auto ten = r / 10;
                auto one = r % 10;
                if (ten > 0) {
                    retarray.push_back(TENS[ten]);
                }
                if (one > 0) {
                    retarray.push_back(MAPPING[one]);
                }
            }
        }
        return join(retarray);
    }

    static string numberToWords(int num) {
        if (num == 0) {
            return "Zero";
        }
        int unit = num % 1000;
        num /= 1000;
        int thousand = num % 1000;
        num /= 1000;
        int million = num % 1000;
        num /= 1000;
        int billion = num;
        std::vector<std::string> retarray;
        if (billion > 0) {
            retarray.push_back(nToWord(billion));
            retarray.emplace_back("Billion");
        }
        if (million > 0) {
            retarray.push_back(nToWord(million));
            retarray.emplace_back("Million");
        }
        if (thousand > 0) {
            retarray.push_back(nToWord(thousand));
            retarray.emplace_back("Thousand");
        }
        if (unit > 0) {
            retarray.push_back(nToWord(unit));
        }
        return join(retarray);
    }
};