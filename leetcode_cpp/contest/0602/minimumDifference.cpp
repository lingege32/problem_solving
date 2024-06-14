#include <algorithm>
#include <array>
#include <cmath>
#include <limits>
#include <vector>
#include <iostream>
using namespace std;

constexpr size_t BITS = 32;
constexpr int MAX = std::numeric_limits<int>::max();
using Arr = std::array<int, BITS>;
class Solution {
  public:
    int minimumDifference(vector<int> &A, int k) {
        int ret = std::numeric_limits<int>::max();
        for (auto a : A) {
            ret = std::min(ret, std::abs(a - k));
        }
        if (ret == 0) {
            return 0;
        }
        auto len = A.size();
        vector<Arr> table;
        vector<Arr> jump;
        table.resize(len + 1, Arr{0});
        jump.resize(len + 1);
        for (auto idx = 1U; idx <= len; ++idx) {
            table[idx] = add(table[idx - 1], transfer(A[idx - 1]));
        }
        auto back = transfer(A.back());
        for (auto i = 0U; i < BITS; ++i) {
            if (back[i]) {
                jump.back()[i] = len;
            } else {
                jump.back()[i] = MAX;
            }
        }
        for (auto i = len - 1; i > 0; --i) {
            auto arr = transfer(A[i - 1]);
            for (auto j = 0U; j < BITS; ++j) {
                if (arr[j]) {
                    jump[i][j] = i;
                } else {
                    jump[i][j] = jump[i+1][j];
                }
            }
        }
        jump[0] = jump[1];

        for (size_t idx=1U; idx<jump.size(); ++idx) {
            auto& jump_arr = jump[idx];
            for (size_t i = 0 ; i<BITS; ++i) {
                auto j_idx = jump_arr[i];
                if (static_cast<size_t>(j_idx) != idx && j_idx != MAX) {
                    auto diff = transfer(minus(table[j_idx], table[idx-1]));
                    // std::cout<<"check idx: "<<idx<<", j_idx: "<<j_idx<<", diff: "<<diff<<"\n";
                    ret = std::min(ret, std::abs(diff-k));
                }
            }
        }
        return ret;
    }
    static Arr transfer(int a) {
        Arr ret{0};
        size_t idx = 0;
        while (a > 0) {
            ret[idx++] = a % 2;
            a >>= 1;
        }
        return ret;
    }

    static int transfer(Arr arr) {
        int ret = 0;
        for (int idx = BITS - 1; idx >= 0; --idx) {
            ret <<= 1;
            ret += (arr[idx]>0);
        }
        return ret;
    }

    static Arr minus(Arr lhs, Arr rhs) {
        for (auto idx = 0U; idx < BITS; ++idx) {
            lhs[idx] -= rhs[idx];
        }
        return lhs;
    }

    static Arr add(Arr lhs, Arr rhs) {
        for (auto idx = 0U; idx < BITS; ++idx) {
            lhs[idx] += rhs[idx];
        }
        return lhs;
    }
};