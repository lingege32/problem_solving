#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static bool isNum(char c) { return c >= '0' && c <= '9'; }

    static string countOfAtoms(string formula) {
        int n = formula.size();
        std::stack<std::map<string, int>> stk;
        std::map<string, int> cur;
        for (int i = 0; i < n;) {
            auto c = formula[i];
            if (c >= 'A' && c <= 'Z') {
                auto next = i + 1;
                for (; next != n && formula[next] >= 'a'; next++) {
                }

                std::string atom = formula.substr(i, next - i);

                auto nextC = formula[next];
                int num = 1;
                if (isNum(nextC)) {
                    auto nextNum = next + 1;
                    for (; nextNum != n && isNum(formula[nextNum]); nextNum++) {
                    }
                    num = std::stoi(formula.substr(next, nextNum - next));
                    next = nextNum;
                }
                cur[atom] += num;
                i = next;
            } else if (c == '(') {
                stk.push(std::move(cur));
                cur.clear();
                i++;
            } else {
                i++;
                if (i != n && isNum(formula[i])) {
                    int next = i + 1;
                    for (; next != n && isNum(formula[next]); next++) {
                    }
                    auto num = std::stoi(formula.substr(i, next - i));
                    for (auto& [ign, cnt] : cur) {
                        cnt *= num;
                    }
                    i = next;
                }
                auto prev = std::move(stk.top());
                stk.pop();
                for (const auto& [s, cnt] : prev) {
                    cur[s] += cnt;
                }
            }
        }
        std::string ret;
        for (const auto& [s, c] : cur) {
            ret += s;
            if (c != 1) {
                ret += std::to_string(c);
            }
        }
        return ret;
    }
};

