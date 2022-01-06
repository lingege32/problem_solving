#include <algorithm>
#include <string>
#include <vector>
using namespace std;

// test1
class Xepic_coding_test1 {
  public:
    bool solve(vector<vector<int>> input) {
        std::sort(input.begin(), input.end(),
                  [](const auto &lhs, const auto &rhs) -> bool {
                      return lhs[0] < rhs[0];
                  });
        // if the schedule is legal,
        // the answer should input[i][1] <= input[i+1][0] 
        for (size_t idx = 1; idx < input.size(); ++idx) {
            if (input[idx][0] < input[idx-1][1]) {
                return false;
            }
        }
        return true;
    }
};

// test 2
struct node {
    int val;
    node *next;
};
class Xepic_coding_test2 {
  public:
    bool solve(node *head) {
        node *slow = head;
        node *fast = head;
        while (slow && fast) {
            slow = slow->next;
            fast = fast->next;
            if (fast) {
                fast = fast->next;
            }
            if (slow == fast) {
                return true;
            }
        }
        return false;
    }
};

// test 3
class Xepic_coding_test3 {
  public:
    int solve(int n) {
        if (n < 3) {
            return n;
        } else {
            vector<int> dp(n, 0);
            dp[0] = 1;
            dp[1] = 2;
            for (int idx = 2; idx < n; ++idx) {
                dp[idx] = dp[idx - 1] + dp[idx - 2];
            }
            return dp[n - 1];
        }
        // unrecheable.
        return 0;
    }
};

// test 4
struct treeNode {
    int val;
    treeNode *left;
    treeNode *right;
};
class Xepic_coding_test4 {
  public:
    treeNode *solve(treeNode *root, int p, int q) {
        if (!root || root->val == p || root->val == q) {
            return root;
        }
        auto lnode = solve(root->left, p, q);
        auto rnode = solve(root->right, p, q);
        if (lnode && rnode) {
            return root;
        }
        return lnode ? lnode : rnode;
    }
};

// test 5
class Xepic_coding_test5 {
  public:
    string solve(string a, string b) {
        // make sure a is shorter than b
        if (a.size() > b.size()) {
            a.swap(b);
        }
        if (a == "0") {
            return b;
        }
        std::reverse(a.begin(), a.end());
        std::reverse(b.begin(), b.end());
        string ans(b.size() + 1, '0');
        size_t idx = 0;
        short carry = 0;
        for (; idx < a.size(); ++idx) {
            int sum = a[idx] - '0' + b[idx] - '0' + carry;
            carry = (sum & 0x2) != 0 ? 1 : 0;
            if (sum & 0x1) {
                ans[idx] = '1';
            }
        }

        for (; idx < b.size(); ++idx) {
            int sum = b[idx] - '0' + carry;
            carry = (sum & 0x2) != 0 ? 1 : 0;
            if (sum & 0x1) {
                ans[idx] = '1';
            }
        }
        if (carry) {
            ans[idx] = '1';
        } else {
            ans.resize(idx);
        }
        std::reverse(ans.begin(), ans.end());
        return ans;
    }
};
