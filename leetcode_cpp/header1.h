#include <iostream>
#include <string>
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


struct Solution {
    int bitwiseComplement(int n);
    vector<vector<string>> partition(string s);
    void solve(vector<vector<char>> &board);
    int uniquePathsIII(vector<vector<int>> &grid);
    int sumNumbers(TreeNode* root);
    int sumOfLeftLeaves(TreeNode* root);
    int arrangeCoins(int n) ;
};


