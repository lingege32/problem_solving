#include <bits/stdc++.h>
using namespace std;

class Solution {
  private:
    static const vector<int> DIRECTION_X;
    static const vector<int> DIRECTION_Y;

    static long long encodePosition(int x, int y) { return ((long long)x << 32) | (y & 0xFFFFFFFFLL); }

    class MobileRobot {
      private:
        int positionX = 0, positionY = 0, orientation = 0;

      public:
        int maxDistanceSquared = 0;

        void executeInstruction(int instruction, const unordered_set<long long>& barriers) {
            if (instruction == -2) {
                orientation = (orientation + 3) % 4;
            } else if (instruction == -1) {
                orientation = (orientation + 1) % 4;
            } else {
                moveForward(instruction, barriers);
            }
        }

      private:
        void moveForward(int steps, const unordered_set<long long>& barriers) {
            for (int i = 0; i < steps; i++) {
                int nextX = positionX + DIRECTION_X[orientation];
                int nextY = positionY + DIRECTION_Y[orientation];
                if (barriers.find(encodePosition(nextX, nextY)) == barriers.end()) {
                    positionX = nextX;
                    positionY = nextY;
                    updateMaxDistance();
                } else {
                    break;
                }
            }
        }

        void updateMaxDistance() {
            maxDistanceSquared = max(positionX * positionX + positionY * positionY, maxDistanceSquared);
        }
    };

  public:
    int robotSim(vector<int>& instructions, vector<vector<int>>& obstacles) {
        MobileRobot robot;
        unordered_set<long long> barrierSet;
        for (const auto& obstacle : obstacles) {
            barrierSet.insert(encodePosition(obstacle[0], obstacle[1]));
        }
        for (int instruction : instructions) {
            robot.executeInstruction(instruction, barrierSet);
        }
        return robot.maxDistanceSquared;
    }
};

const vector<int> Solution::DIRECTION_X = {0, 1, 0, -1};
const vector<int> Solution::DIRECTION_Y = {1, 0, -1, 0};