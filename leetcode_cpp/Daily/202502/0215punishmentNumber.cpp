#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
constexpr int N = 29;

bool check(int base, int total, int cur) {
    if (total == 0) {
        return cur == base;
    }
    for (int i = 10; i < 100000000; i *= 10) {
        bool cc = check(base, total / i, cur + (total % i));
        if (cc) {
            return true;
        }
    }
    return false;
}

std::pair<std::array<int, N>, std::array<int, N>> getArray() {
    std::pair<std::array<int, N>, std::array<int, N>> ret;
    auto& first = ret.first;
    auto& second = ret.second;
    first.fill(std::numeric_limits<int>::max());
    second.fill(std::numeric_limits<int>::max());
    first[0] = 1;
    first[1] = 9;
    first[2] = 10;
    second[0] = 1;
    second[1] = 81;
    second[2] = 100;
    int idx = 3;
    for (int i = 11; i <= 1000; i++) {
        int v = i * i;
        bool c = check(i, v, 0);
        if (c) {
            first[idx] = i;
            second[idx] = v;
            idx++;
        }
    }
    return ret;
}

auto& getTable() {
    static auto ret = getArray();
    return ret;
}

class Solution {
  public:
    static int punishmentNumber(int n) {
        auto& table = getTable();
        int ret = 0;
        for (int i = 0; i < N; ++i) {
            auto val = table.first[i];
            if (val > n) {
                break;
            }
            ret += table.second[i];
        }
        return ret;
    }
};