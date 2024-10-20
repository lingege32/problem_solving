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
    static bool parseBoolExpr(const string& expression) {
        std::vector<std::pair<char, std::vector<bool>>> stack;
        char op = 0;
        std::vector<bool> cur;
        for (auto c : expression) {
            if (c == ',') {
                continue;
            }
            if (c == '(') {
                stack.emplace_back(op, std::move(cur));
            } else if (c == ')') {
                auto [oper, vec] = stack.back();
                stack.pop_back();
                bool res = false;
                if (oper == '!') {
                    res = !cur[0];
                } else if (oper == '&') {
                    res = true;
                    for (auto b : cur) {
                        res &= b;
                    }
                } else if (oper == '|') {
                    res = false;
                    for (auto b : cur) {
                        res |= b;
                    }
                }
                vec.push_back(res);
                cur.swap(vec);
            } else if (c == 't') {
                cur.push_back(true);

            } else if (c == 'f') {
                cur.push_back(false);
            } else {
                op = c;
            }
        }
        return cur.empty() ? false : cur[0];
    }
};

class OptSolution {
  public:
    bool andd(string exp) {
        int n = exp.length(), j = 0;
        string subexp;
        int active = 0;
        while (j < n) {
            if (active == 0 && exp[j] == ',') {
                if (!parseBoolExpr(subexp)) {
                    return false;
                }
                subexp = "";
                j++;
                continue;
            }
            if (exp[j] == '(') {
                active++;
            }
            if (exp[j] == ')') {
                active--;
            }
            subexp += exp[j++];
        }
        return parseBoolExpr(subexp);
    }

    bool orr(string exp) {
        int n = exp.length(), j = 0;
        string subexp;
        int active = 0;
        while (j < n) {
            if (active == 0 && exp[j] == ',') {
                if (parseBoolExpr(subexp)) {
                    return true;
                }
                subexp = "";
                j++;
                continue;
            }
            if (exp[j] == '(') {
                active++;
            }
            if (exp[j] == ')') {
                active--;
            }
            subexp += exp[j++];
        }
        return parseBoolExpr(subexp);
    }

    bool parseBoolExpr(string exp) {
        int n = exp.length();
        if (n == 1) {
            return (exp[0] == 't');
        }
        if (exp[0] == '!') {
            return !parseBoolExpr(exp.substr(2, n - 3));
        }
        if (exp[0] == '&') {
            return andd(exp.substr(2, n - 3));
        }
        if (exp[0] == '|') {
            return orr(exp.substr(2, n - 3));
        }
        return false;
    }
};