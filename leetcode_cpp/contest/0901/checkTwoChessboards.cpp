#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    bool checkTwoChessboards(string coordinate1, string coordinate2) {
        auto a1 = coordinate1[0] - 'a';
        auto a2 = coordinate1[1] - '1';
        auto b1 = coordinate2[0] - 'a';
        auto b2 = coordinate2[1] - '1';
        return (a1 + a2) % 2 == (b1 + b2) % 2;
    }
};