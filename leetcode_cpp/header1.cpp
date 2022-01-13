#include "header1.h"
#include <algorithm>
#include <iterator>
#include <memory.h>
#include <string_view>

int Solution::bitwiseComplement(int n) {
    if (n == 0) {
        return 1;
    }
    int ans = 0;
    int mask = 1 << 30;
    while (mask > 0 && (mask & n) == 0) {
        mask >>= 1;
    }
    //   std::cout<<(void*)mask<<", "<<(void*)n<<std::endl;
    while (mask > 0) {
        ans <<= 1;
        if ((mask & n) == 0) {
            ans += 1;
        }
        mask >>= 1;
    }
    //   std::cout<<ans<<std::endl;
    return ans;
}

vector<vector<string>> Solution::partition(string s) {
    struct Solution2 {
        bool dp[20][20];
        vector<string> curList;
        int n;

      public:
        vector<vector<string>> partition(string s) {
            vector<vector<string>> ans;
            n = s.length();

            memset(dp, false, sizeof dp);
            dfs(ans, s, 0);

            return ans;
        }

        void dfs(vector<vector<string>> &ans, string &s, int start) {
            if (start >= n) {
                ans.emplace_back(curList);
            }

            for (int i = start; i < n; i++) {
                if (s[start] == s[i] &&
                    (i - start <= 2 || dp[start + 1][i - 1])) {
                    dp[start][i] = true;
                    curList.emplace_back(s.substr(start, i - start + 1));
                    dfs(ans, s, i + 1);
                    curList.pop_back();
                }
            }
        }
    };
    Solution2 s2;
    return s2.partition(std::move(s));
}

void Solution::solve(vector<vector<char>> &board) {
    struct Solution2 {
      public:
        int go[4][2] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

        bool isSafe(int i, int j, int n, int m) {
            return (i > 0 && i < n && j > 0 && j < m);
        }

        void dfs(vector<vector<char>> &a, int i, int j, int n, int m) {
            a[i][j] = 'B';

            for (int k = 0; k < 4; k++) {
                int newi = i + go[k][0];
                int newj = j + go[k][1];

                if (isSafe(newi, newj, n, m) && a[newi][newj] == 'O') {
                    dfs(a, newi, newj, n, m);
                }
            }
        }

        void solve(vector<vector<char>> &a) {
            int n = a.size();
            int m = a[0].size();

            for (int i = 0; i < n; i++) {
                if (a[i][0] == 'O') {
                    dfs(a, i, 0, n, m);
                }
            }
            for (int i = 0; i < n; i++) {
                if (a[i][m - 1] == 'O') {
                    dfs(a, i, m - 1, n, m);
                }
            }
            for (int j = 0; j < m; j++) {
                if (a[0][j] == 'O') {
                    dfs(a, 0, j, n, m);
                }
            }
            for (int j = 0; j < m; j++) {
                if (a[n - 1][j] == 'O') {
                    dfs(a, n - 1, j, n, m);
                }
            }

            for (int i = 0; i < n; i++) {
                for (int j = 0; j < m; j++) {
                    if (a[i][j] == 'O') {
                        a[i][j] = 'X';
                    }
                }
            }
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < m; j++) {
                    if (a[i][j] == 'B') {
                        a[i][j] = 'O';
                    }
                }
            }
        }
    };
    Solution2 s2;
    return s2.solve(board);
}

int Solution::uniquePathsIII(vector<vector<int>> &grid) {
    struct Solution2 {
        vector<vector<short>> visit;
        int path;
        int count;
        int current_count;
        int end_r, end_c;

        bool isSafe(vector<vector<int>> &grid, int r, int c, int row, int col) {
            return r >= 0 && c >= 0 && r < row && c < col && grid[r][c] != -1;
        }
        int uniquePathsIII(vector<vector<int>> &grid) {
            int row = grid.size();
            int col = grid[0].size();
            visit = vector<vector<short>>(row, vector<short>(col, 0));
            path = row * col;
            int start_r = 0;
            int start_c = 0;
            end_r = 0;
            end_c = 0;
            count = 0;
            current_count = 0;
            for (size_t r = 0; r < row; ++r) {
                for (size_t c = 0; c < col; ++c) {
                    if (grid[r][c] != 0) {
                        if (grid[r][c] == 1) {
                            start_r = r;
                            start_c = c;
                        } else if (grid[r][c] == -1) {
                            path -= 1;
                        } else {
                            end_r = r;
                            end_c = c;
                        }
                    }
                }
            }
            dfs(grid, start_r, start_c, row, col);
            return count;
        }
        void dfs(vector<vector<int>> &grid, int r, int c, int row, int col) {
            if (visit[r][c] == 1) {
                return;
            }
            if (r == end_r && c == end_c && current_count + 1 == path) {
                count++;
            }
            visit[r][c] = 1;
            current_count++;

            if (isSafe(grid, r - 1, c, row, col)) {
                dfs(grid, r - 1, c, row, col);
            }
            if (isSafe(grid, r + 1, c, row, col)) {
                dfs(grid, r + 1, c, row, col);
            }
            if (isSafe(grid, r, c - 1, row, col)) {
                dfs(grid, r, c - 1, row, col);
            }
            if (isSafe(grid, r, c + 1, row, col)) {
                dfs(grid, r, c + 1, row, col);
            }
            visit[r][c] = 0;
            current_count--;
        }
    };
    Solution2 s2;
    return s2.uniquePathsIII(grid);
}

int Solution::sumNumbers(TreeNode *root) {
    struct Solution2 {
        int sum = 0;
        int sumNumbers(TreeNode *root) {
            dfs(root, 0);
            return sum;
        }
        void dfs(TreeNode *root, int cur) {
            if (root) {
                cur *= 10;
                cur += root->val;
                if (!root->left && !root->right) {
                    sum += cur;
                } else {
                    dfs(root->left, cur);
                    dfs(root->right, cur);
                }
            }
        }
    };
    Solution2 s2;
    return s2.sumNumbers(root);
}

int Solution::sumOfLeftLeaves(TreeNode *root) {
    struct Solution2 {
        int sum = 0;
        int sumOfLeftLeaves(TreeNode *root) {
            dfs(root, false);
            return sum;
        }
        void dfs(TreeNode *root, bool left) {
            if (root) {
                if (!root->left && !root->right && left) {
                    sum += root->val;
                } else {
                    dfs(root->left, true);
                    dfs(root->right, false);
                }
            }
        }
    };
    Solution2 s2;
    return s2.sumOfLeftLeaves(root);
}

int Solution::arrangeCoins(int n) {
    long lo = 1;
    long left = lo;
    long right = n;
    while (1) {
        long hi = (left + right) / 2;
        long area = (1 + hi) * hi / 2;
        if (n >= area && n < area + hi + 1) {
            return hi;
        }
        if (area > n) {
            right = hi;
        } else {
            left = hi + 1;
        }
    }
}

vector<int> Solution::singleNumber(vector<int> &nums) {
    // one assumption is exact two answer
    int bit = 0;
    for (auto i : nums) {
        bit ^= i;
    }

    int which_bit = 0;
    for (int i = 0; i < 32; ++i) {
        if (bit & (1 << i)) {
            which_bit = i;
            break;
        }
    }
    int first = 0;
    for (auto i : nums) {
        if ((i & (1 << which_bit)) == 0) {
            first ^= i;
        }
    }
    int second = first ^ bit;
    return {first, second};
}

string Solution::multiply(string num1, string num2) {
    if ((num1.size() == 1 && num1[0] == '0') ||
        (num2.size() == 1 && num2[0] == '0')) {
        return "0";
    }
    string ans(num1.size() + num2.size(), '0');
    if (num1.size() < num2.size()) {
        // make num1.size > num2.size
        num1.swap(num2);
    }
    std::reverse(num1.begin(), num1.end());
    std::reverse(num2.begin(), num2.end());
    auto add = [](char a, char b, char carry) -> array<char, 2> {
        array<char, 2> ans;
        int c = a - '0' + b - '0' + carry - '0';

        ans[0] = (c % 10) + '0';
        ans[1] = (c / 10) + '0';
        return ans;
    };
    auto mul = [](char a, char b) -> array<char, 2> {
        array<char, 2> ans;
        int c = (a - '0') * (b - '0');
        ans[0] = (c % 10) + '0';
        ans[1] = (c / 10) + '0';
        return ans;
    };
    for (size_t i = 0; i < num1.size(); ++i) {
        char char1 = num1[i];
        string tmp(ans.size(), '0');
        int carry = 0;
        for (size_t j = 0; j < num2.size(); ++j) {
            char char2 = num2[j];
            auto tmp_mul = mul(char1, char2);
            tmp[j + i] = tmp_mul[0] + carry;
            carry = tmp_mul[1] - '0';
        }
        if (carry != 0) {
            tmp[num2.size() + i] = carry + '0';
        }
        carry = 0;
        for (size_t k = 0; k < tmp.size(); ++k) {
            auto add_num = add(ans[k], tmp[k], '0' + carry);
            ans[k] = add_num[0];
            carry = add_num[1] - '0';
        }
    }
    int l = ans.size() - 1;
    while (l >= 0) {
        if (ans[l] != '0') {
            break;
        }
        --l;
    }
    ans.resize(l + 1);
    std::reverse(ans.begin(), ans.end());
    return ans;
}

bool Solution::carPooling(vector<vector<int>> &trips, int capacity) {
    int trip[1001] = {0};

    int last_in = 0;
    for (auto &aTrip : trips) {
        trip[aTrip[1]] += aTrip[0];
        trip[aTrip[2]] -= aTrip[0];
        last_in = max(last_in, aTrip[1]);
    }
    int passengers = 0;
    for (int idx = 0; idx <= last_in; ++idx) {
        passengers += trip[idx];
        if (passengers > capacity) {
            return false;
        }
    }

    // this is slower algorithm
    // for (auto& aTrip : trips) {
    //     for (int idx = aTrip[1]; idx<aTrip[2]; ++idx) {
    //         trip[idx] += aTrip[0];
    //         if (trip[idx] > capacity) {
    //             return false;
    //         }
    //     }
    // }
    return true;
}

int Solution::numTrees(int n) {

    struct Solution2 {
        vector<int> dp;
        int find_dp_n(int n) {
            if (dp[n] == -1) {
                for (int left = 0; left < n; ++left) {
                    int right = n - left - 1;
                    dp[n] += find_dp_n(left) * find_dp_n(right);
                }
            }
            return dp[n];
        }
        int numTrees(int n) {
            dp = vector<int>(n + 1, -1);

            dp[0] = 1;
            dp[1] = 1;
            dp[2] = 2;
            dp[3] = 5;

            return find_dp_n(n);
        }
    };
    Solution2 s2;
    return s2.numTrees(n);
}

int Solution::maxProfit(vector<int> &prices) {
    int ans = 0;
    for (int idx = 1; idx < prices.size(); ++idx) {
        int left = prices[idx - 1];
        int right = prices[idx];
        if (right > left) {
            ans += (right - left);
        }
    }
    return ans;
}

int Solution::minStartValue(vector<int> &nums) {
    int min = 0;
    int total = 0;
    for (int n : nums) {
        total += n;
        min = std::min(min, total);
    }
    return min < 0 ? -min + 1 : 1;
}

ListNode *Solution::removeElements(ListNode *head, int val) {
    ListNode dummy;
    dummy.next = head;
    ListNode **h = &dummy.next;
    while (*h) {
        if ((*h)->val == val) {
            *h = (*h)->next;
        } else {
            h = &(*h)->next;
        }
    }
    return dummy.next;
}

vector<int> Solution::dailyTemperatures(vector<int> &temperatures) {
    vector<int> &temp = temperatures;
    vector<int> ans(temp.size(), 0);
    int max = temp.back();
    for (int idx = ans.size() - 2; idx >= 0; --idx) {
        if (temp[idx] > max) {
            max = temp[idx];
            continue;
        } else {
            int target = idx + 1;
            while (1) {
                if (temp[target] > temp[idx]) {
                    ans[idx] = target - idx;
                    break;
                } else {
                    if (ans[target] == 0) {
                        break;
                    } else {
                        target += ans[target];
                    }
                }
            }
        }
    }
    return ans;
}

bool Solution::isMatch(string s, string p) {

    size_t slen = s.size();
    size_t plen = p.size();
    vector<vector<bool>> dp(slen + 1, vector<bool>(plen + 1, false));
    dp[0][0] = true;

    for (size_t sidx = 0; sidx <= slen; ++sidx) {
        for (size_t pidx = 1; pidx <= plen; ++pidx) {
            if (pidx > 1 && p[pidx - 1] == '*') {
                dp[sidx][pidx] = dp[sidx][pidx - 2] ||
                                 (sidx > 0 && (dp[sidx - 1][pidx] &&
                                               (s[sidx - 1] == p[pidx - 2] ||
                                                p[pidx - 2] == '.')));
            } else {
                dp[sidx][pidx] =
                    sidx > 0 && dp[sidx - 1][pidx - 1] &&
                    (s[sidx - 1] == p[pidx - 1] || p[pidx - 1] == '.');
            }
        }
    }
    return dp[slen][plen];
}
int Solution::cherryPickup(vector<vector<int>> &grid) {

    size_t row = grid.size();
    size_t col = grid[0].size();
    int dp[70][71][71];
    memset(dp, -1, sizeof(dp));
    dp[0][0][col - 1] = grid[0][0] + grid[0][col - 1];
    for (size_t r = 1; r < row; r++) {
        for (int c1 = 0; c1 < col; c1++) {
            for (int c2 = 0; c2 < col; c2++) {
                int last = -1;

                for (int c1_tmp = c1 - 1; c1_tmp <= c1 + 1; ++c1_tmp) {
                    for (int c2_tmp = c2 - 1; c2_tmp <= c2 + 1; ++c2_tmp) {

                        if (c1_tmp >= 0 && c1_tmp < col && c2_tmp >= 0 &&
                            c2_tmp < col) {
                            last = std::max(last, dp[r - 1][c1_tmp][c2_tmp]);
                        }
                    }
                }
                if (last == -1)
                    continue;
                if (c1 == c2) {
                    dp[r][c1][c2] = last + grid[r][c1];
                } else {
                    dp[r][c1][c2] = last + grid[r][c1] + grid[r][c2];
                }
            }
        }
    }
    int ans = 0;
    for (size_t c1 = 0; c1 < col; ++c1) {
        for (size_t c2 = 0; c2 < col; ++c2) {
            if (c1 != c2) {
                ans = std::max(ans, dp[row - 1][c1][c2]);
            }
        }
    }

    return ans;
}

bool Solution::isRobotBounded(string instructions) {
    int x = 0;
    int y = 0;
    int d = 0;
    vector<int> dx{0, -1, 0, 1};
    vector<int> dy{1, 0, -1, 0};
    for (char c : instructions) {
        if (c == 'G') {
            x += dx[d];
            y += dy[d];
        } else {
            d = (d + (c == 'L' ? 1 : 3)) % 4;
        }
    }
    return x == 0 && y == 0 || d;
}
string Solution::addBinary(string a, string b) {
    if (a == "0") {
        return b;
    } else if (b == "0") {
        return a;
    }
    if (a.size() > b.size()) {
        a.swap(b);
    }

    std::reverse(a.begin(), a.end());
    std::reverse(b.begin(), b.end());

    string ans(b.size() + 1, '0');
    short carry = 0;
    int idx = 0;
    for (; idx < a.size(); ++idx) {
        short t = a[idx] - '0' + b[idx] - '0' + carry;
        carry = (t & 0x2) ? 1 : 0;
        ans[idx] = (t & 0x1) ? '1' : '0';
    }
    for (; idx < b.size(); ++idx) {
        short t = b[idx] - '0' + carry;
        carry = (t & 0x2) ? 1 : 0;
        ans[idx] = (t & 0x1) ? '1' : '0';
    }

    if (carry == 1) {
        ans[idx] = '1';
    } else {
        ans.resize(idx);
    }
    std::reverse(ans.begin(), ans.end());
    return ans;
}

void sumRootToLeafDfs(TreeNode *root, int tmp, int &ans) {
    if (root) {
        int cur_total = tmp * 2 + root->val;
        if (!root->left && !root->right) {
            ans += cur_total;
        } else {
            sumRootToLeafDfs(root->left, cur_total, ans);
            sumRootToLeafDfs(root->right, cur_total, ans);
        }
    }
}
int Solution::sumRootToLeaf(TreeNode *root) {
    int ans = 0;
    sumRootToLeafDfs(root, 0, ans);
    return ans;
}

TreeNode *Solution::insertIntoBST(TreeNode *root, int val) {
    if (!root) {
        return new TreeNode(val);
    }
    if (val > root->val) {
        root->right = insertIntoBST(root->right, val);
    } else {
        root->left = insertIntoBST(root->left, val);
    }
    return root;
}

bool Solution::isWildCardMatch(string s, string p) {
    int pPtr = 0;
    int sPtr = 0;

    int starPos = -1;
    int starTextPos = -1;

    while (sPtr < s.size()) {
        // cout << sPtr << " " << s[sPtr] << " " << pPtr << " " << p[pPtr] <<
        // endl;

        if (p[pPtr] == '*') {
            starPos = pPtr;
            starTextPos = sPtr;
            ++pPtr;

            if (pPtr >= p.size()) {
                return true;
            }

        } else if (p[pPtr] == '?' || p[pPtr] == s[sPtr]) {
            ++pPtr;
            ++sPtr;
        } else {
            // rollback
            if (starPos == -1) {
                return false;
            }
            pPtr = starPos + 1;
            // try one more character
            ++starTextPos;
            sPtr = starTextPos;
        }
    }

    while (pPtr < p.size()) {
        if (p[pPtr] != '*') {
            return false;
        }
        ++pPtr;
    }
    return true;

    // dp is not good
    // const size_t slen = s.size();
    // const size_t plen = p.size();
    // vector<bool> dp((slen + 1) * (plen + 1), false);
    // auto take = [&](size_t r, size_t c) -> bool {
    //     return dp[r * (plen + 1) + c];
    // };
    // auto modify = [&](size_t r, size_t c, bool val) {
    //     dp[r * (plen + 1) + c] = val;
    // };

    // modify(0, 0, true);
    // for (size_t r = 0; r <= slen; ++r) {
    //     for (size_t c = 1; c <= plen; ++c) {
    //         if (p[c - 1] == '*') {
    //             modify(r, c, take(r, c - 1) || (r > 0 && take(r - 1, c)));
    //         } else {
    //             modify(r, c,
    //                    r > 0 && take(r - 1, c - 1) &&
    //                        (s[r - 1] == p[c - 1] || p[c - 1] == '?'));
    //         }
    //     }
    // }
    // return take(slen, plen);
}

int Solution::findMinArrowShots(vector<vector<int>> &points) {
    if (points.empty()) {
        return 0;
    }

    using point = pair<int, int>;
    vector<point> pts;
    pts.reserve(points.size());
    for (const auto &p : points) {
        pts.emplace_back(p[0], p[1]);
    }
    std::sort(pts.begin(), pts.end(),
              [](const pair<int, int> &lhs, const pair<int, int> &rhs) {
                  return lhs.first == rhs.first ? lhs.second < rhs.second
                                                : lhs.first < rhs.first;
              });
    int arrows = 1;
    int arrow = pts[0].second;

    for (size_t idx = 1; idx < pts.size(); ++idx) {
        auto [s, e] = pts[idx];
        arrow = min(arrow, e);
        if (arrow < s) {
            arrow = e;
            arrows++;
        }
    }

    return arrows;
}