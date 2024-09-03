#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
  int finalPositionOfSnake(int n, vector<string> &commands) {
    int ret = 0;
    for (const auto &command : commands) {
      if (command == "UP") {
        ret -= n;
      } else if (command == "RIGHT") {
        ret += 1;
      } else if (command == "LEFT") {
        ret -= 1;
      } else if (command == "DOWN") {
        ret += n;
      }
    }
    return ret;
  }
};