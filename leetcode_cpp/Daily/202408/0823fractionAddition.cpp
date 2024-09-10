#include <bits/stdc++.h>
using namespace std;

struct Num {
    int top = 0;
    int down = 0;
    bool positive = true;
    friend std::ostream& operator<<(std::ostream& os, const Num& f);
};

std::ostream& operator<<(std::ostream& os, const Num& f) {
    if (!f.positive) {
        os << '-';
    }
    os << f.top << '/' << f.down;
    return os;
}

class Solution {
  public:
    static string fractionAddition(const string& expression) {
        Num cur;
        bool isTop = true;
        std::vector<Num> nums;
        for (auto c : expression) {
            if (c == '+' || c == '-') {
                if (cur.down != 0) {
                    nums.push_back(cur);
                }
                cur = Num();
                cur.positive = (c == '+');
                isTop = true;
            } else if (c == '/') {
                isTop = false;
            } else {
                auto add = [](int& n, char c) {
                    n *= 10;
                    n += (c - '0');
                };
                if (isTop) {
                    add(cur.top, c);
                } else {
                    add(cur.down, c);
                }
            }
        }
        nums.push_back(cur);
        // for (const auto& num : nums) {
        //     std::cout << num << std::endl;
        // }
        auto lcm = 1;
        for (const auto& num : nums) {
            lcm = std::lcm(lcm, num.down);
        }

        // std::cout << "lcm: " << lcm << std::endl;
        auto top = 0;
        for (const auto& num : nums) {
            auto t = (num.positive ? 1 : -1) * num.top * (lcm / num.down);
            top += t;
        }
        std::string res;
        if (top < 0) {
            top = -top;
            res.push_back('-');
        }
        auto gcd = std::gcd(top, lcm);
        top /= gcd;
        lcm /= gcd;
        res += std::to_string(top);
        res += "/";
        res += std::to_string(lcm);
        return res;
    }
};