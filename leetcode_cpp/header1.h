#include <algorithm>
#include <array>
#include <climits>
#include <cstring>
#include <iostream>
#include <limits>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Node {
  public:
    int val;
    vector<Node *> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node *>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node *>();
    }
    Node(int _val, vector<Node *> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

struct Solution {
    int bitwiseComplement(int n);
    vector<vector<string>> partition(string s);
    void solve(vector<vector<char>> &board);
    int uniquePathsIII(vector<vector<int>> &grid);
    int sumNumbers(TreeNode *root);
    int sumOfLeftLeaves(TreeNode *root);
    int arrangeCoins(int n);
    vector<int> singleNumber(vector<int> &nums);
    string multiply(string num1, string num2);
    bool carPooling(vector<vector<int>> &trips, int capacity);
    int numTrees(int n);
    int maxProfit(vector<int> &prices);
    int minStartValue(vector<int> &nums);
    ListNode *removeElements(ListNode *head, int val);
    vector<int> dailyTemperatures(vector<int> &temperatures);
    bool isMatch(string s, string p);
    int cherryPickup(vector<vector<int>> &grid);
    bool isRobotBounded(string instructions);
    string addBinary(string a, string b);
    int sumRootToLeaf(TreeNode *root);
    TreeNode *insertIntoBST(TreeNode *root, int val);
    bool isWildCardMatch(string s, string p);
    int findMinArrowShots(vector<vector<int>> &points);
    int minCostConnectPoints(vector<vector<int>> &points);
    int myAtoi(string s);
    int minJumps(vector<int> &arr);
    int maxDistToClosest(vector<int> &seats);
    Node *cloneGraph(Node *node);
    bool wordPattern(string pattern, string s);
    ListNode *detectCycle(ListNode *head);
    bool canPlaceFlowers(vector<int> &flowerbed, int n);
    int minEatingSpeed(vector<int> &piles, int h);
    int canCompleteCircuit(vector<int> &gas, vector<int> &cost);
    bool winnerSquareGame(int n);
    vector<int> sequentialDigits(int low, int high);
    bool detectCapitalUse(string word);
    bool validMountainArray(vector<int> &arr);
    vector<int> getAllElements(TreeNode* root1, TreeNode* root2);
};

class CombinationIterator {
    string mcharacters;
    bool hasNextFlag;
    string ans;
    vector<int> arrange;

  public:
    CombinationIterator(string characters, int combinationLength)
        : mcharacters(std::move(characters)), hasNextFlag(true),
          ans(mcharacters.begin(), mcharacters.begin() + combinationLength),
          arrange(combinationLength, 0) {
        for (int i = 0; i < arrange.size(); ++i) {
            arrange[i] = i;
        }
    }

    string next() {
        string tmp = ans;
        if (arrange[0] == mcharacters.size() - ans.size()) {
            hasNextFlag = false;
        } else {
            int idx = ans.size() - 1;
            for (; idx >= 0; --idx) {
                int dis = ans.size() - idx;
                if (arrange[idx] + dis != mcharacters.size()) {
                    break;
                }
            }
            int new_val = arrange[idx] + 1;
            for (int i = idx; i < arrange.size(); ++i) {
                arrange[i] = new_val++;
            }
            for (; idx < arrange.size(); ++idx) {
                ans[idx] = mcharacters[arrange[idx]];
            }
        }
        return tmp;
    }

    bool hasNext() { return hasNextFlag; }
};