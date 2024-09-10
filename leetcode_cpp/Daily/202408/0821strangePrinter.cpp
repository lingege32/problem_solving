#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    template <typename T, size_t N>
    struct TwoDimArray {
        std::array<T, N * N> data;

        T* operator[](size_t k) { return &data[k * N]; }
    };

    static int strangePrinter(string s) {
        int n = s.length();
        vector<int> prevs(n, -1);
        std::array<int, 26> char_prev;
        char_prev.fill(-1);
        for (int i = 0; i < n; i++) {
            int key = s[i] - 'a';
            prevs[i] = char_prev[key];
            char_prev[key] = i;
        }

        TwoDimArray<int, 100> min_turns;
        min_turns[0][0] = 1;
        for (int end = 1; end < n; end++) {
            min_turns[end][end] = 1;
            for (int start = end - 1; start >= 0; start--) {
                if (prevs[end] == end - 1) {
                    min_turns[start][end] = min_turns[start][end - 1];
                } else if (prevs[end] < start) {
                    min_turns[start][end] = min_turns[start][end - 1] + 1;
                } else {
                    min_turns[start][end] = min_turns[start][end - 1] + 1;
                    int prev = prevs[end];
                    while (prev >= start) {
                        min_turns[start][end] = min(min_turns[start][end],
                                                    min_turns[start][prev] + min_turns[prev + 1][end - 1]);
                        prev = prevs[prev];
                    }
                }
            }
        }
        return min_turns[0][n - 1];
    }
};