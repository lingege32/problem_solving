#include "header1.h"
#include <algorithm>
#include <cctype>
#include <iterator>
#include <numeric>

int Solution::maxDistToClosest(vector<int> &seats) {
    int left_sitting = 0;
    for (; seats[left_sitting] == 0; ++left_sitting) {
    }
    int max_distance = left_sitting;
    int ss = seats.size();
    for (int right_sitting = left_sitting + 1; right_sitting < ss;
         ++right_sitting) {
        if (seats[right_sitting] == 1) {
            max_distance =
                std::max(max_distance, (right_sitting - left_sitting) / 2);
            left_sitting = right_sitting;
        }
    }
    for (int i = ss - 1; i >= 0; --i) {
        if (seats[i] == 1) {
            max_distance = std::max(max_distance, ss - 1 - i);
            break;
        }
    }

    return max_distance;
}

Node *Solution::cloneGraph(Node *node) {
    unordered_set<Node *> done;
    unordered_map<Node *, Node *> table;
    queue<Node *> q;
    table[node] = new Node(node->val);
    q.push(node);

    while (!q.empty()) {
        Node *n = q.front();
        q.pop();
        if (done.find(n) != done.end()) {
            continue;
        } else {
            done.insert(n);
        }

        Node *new_cur = table[n];
        for (auto neighbor : n->neighbors) {
            q.push(neighbor);
            auto neiIter = table.find(neighbor);
            Node *new_neighbor = nullptr;
            if (neiIter == table.end()) {
                new_neighbor = new Node(neighbor->val);
                table[neighbor] = new_neighbor;
            } else {
                new_neighbor = neiIter->second;
            }
            new_cur->neighbors.push_back(new_neighbor);
        }
    }
    return table[node];
}

bool Solution::wordPattern(string pattern, string s) {
    unordered_map<char, string> m;
    istringstream in(s);
    int i = 0, n = pattern.size();
    for (string word; in >> word; ++i) {
        if (i >= n)
            continue;
        if (m.count(pattern[i])) {
            if (m[pattern[i]] != word)
                return false;
        } else {
            for (auto &a : m) {
                if (a.second == word)
                    return false;
            }
            m[pattern[i]] = word;
        }
    }
    return i == n;
}

ListNode *Solution::detectCycle(ListNode *head) {
    ListNode *slow = head;
    ListNode *fast = head;
    while (fast) {
        slow = slow->next;
        fast = fast->next;
        if (fast) {
            fast = fast->next;
        }
        if (fast && slow == fast) {
            break;
        }
    }
    if (fast) {
        slow = head;
        while (slow != fast) {
            slow = slow->next;
            fast = fast->next;
        }
        return slow;
    }
    return nullptr;
}
bool Solution::canPlaceFlowers(vector<int> &flowerbed, int n) {
    auto it1 = std::find(flowerbed.begin(), flowerbed.end(), 1);
    if (it1 == flowerbed.end())
        return (flowerbed.size() + 1) / 2 >= n;
    int max_flowers = std::distance(flowerbed.begin(), it1) / 2;

    auto it0 = std::find(it1, flowerbed.end(), 0);
    it1 = std::find(it0, flowerbed.end(), 1);
    while (it1 != flowerbed.end()) {
        max_flowers += (std::distance(it0, it1) - 1) / 2;
        it0 = std::find(it1, flowerbed.end(), 0);
        it1 = std::find(it0, flowerbed.end(), 1);
    }
    max_flowers += (std::distance(it0, it1) / 2);
    return max_flowers >= n;
}

int Solution::minEatingSpeed(vector<int> &piles, int h) {
    int left = 1;
    int right = *std::max_element(piles.begin(), piles.end());
    while (left < right) {
        int mid = (left + right) / 2;
        int needHr = 0;
        for (auto banana : piles) {

            needHr += ((banana / mid) + ((banana % mid) == 0 ? 0 : 1));
            if (needHr > h) {
                break;
            }
        }
        if (needHr > h) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left;
}

int Solution::canCompleteCircuit(vector<int> &gas, vector<int> &cost) {
    const int n = gas.size();
    int sum = 0, min_sum = std::numeric_limits<int>::max(), min_i = -1;
    for (int i = 0; i < n; ++i) {
        sum += gas[i] - cost[i];
        if (sum < min_sum) {
            min_sum = sum;
            min_i = i;
        }
    }
    return sum >= 0 ? ((min_i + 1) % n) : -1;
}

bool Solution::winnerSquareGame(int n) {
    bool dp[100001];
    std::memset(dp, false, sizeof(dp));
    for (int i = 0; i <= n; ++i) {
        if (dp[n] == true)
            break;
        if (!dp[i]) {
            for (int j = 1; j * j + i <= n; ++j)
                dp[j * j + i] = true;
        }
    }

    return dp[n];
}

vector<int> Solution::sequentialDigits(int low, int high) {
    int buffer[] = {12,      23,      34,      45,       56,       67,
                    78,      89,      123,     234,      345,      456,
                    567,     678,     789,     1234,     2345,     3456,
                    4567,    5678,    6789,    12345,    23456,    34567,
                    45678,   56789,   123456,  234567,   345678,   456789,
                    1234567, 2345678, 3456789, 12345678, 23456789, 123456789};
    vector<int> ans;
    for (int i = 0; i < sizeof(buffer) / sizeof(int); ++i) {
        int tmp = buffer[i];
        if (tmp > high) {
            break;
        } else if (tmp >= low) {
            ans.push_back(tmp);
        }
    }
    return ans;
}

bool Solution::detectCapitalUse(string word) {
    if (word.size() <= 1) {
        return true;
    } else {
        short strategy = 0;
        if (isupper(word[0])) {
            strategy |= 0x1;
        }
        if (islower(word[1])) {
            strategy |= 0x2;
        }
        if (strategy == 1) {
            return std::all_of(word.begin() + 2, word.end(),
                               [](char c) { return isupper(c); });
        } else if (strategy == 2) {
            return std::all_of(word.begin() + 2, word.end(),
                               [](char c) { return islower(c); });
        } else if (strategy == 3) {
            return std::all_of(word.begin() + 2, word.end(),
                               [](char c) { return islower(c); });
        } else {
            return false;
        }
    }
    // unreachable
    return false;
}

bool Solution::validMountainArray(vector<int> &arr) {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    auto it_mountain = std::max_element(arr.begin(), arr.end());
    auto mid = std::distance(arr.begin(), it_mountain);
    if (mid == 0 || mid == arr.size() - 1) {
        return false;
    }
    for (size_t idx = 1; idx <= mid; ++idx) {
        if (arr[idx - 1] >= arr[idx]) {
            return false;
        }
    }
    for (size_t idx = mid + 1; idx < arr.size(); ++idx) {
        if (arr[idx - 1] <= arr[idx]) {
            return false;
        }
    }
    return true;
}

vector<int> Solution::getAllElements(TreeNode *root1, TreeNode *root2) {
    std::function<void(TreeNode *, vector<int> &)> in_order_traverse;
    in_order_traverse = [&](TreeNode *node, vector<int> &v_in_order_traverse) {
        if (!node) {
            return;
        }
        in_order_traverse(node->left, v_in_order_traverse);
        v_in_order_traverse.push_back(node->val);
        in_order_traverse(node->right, v_in_order_traverse);
    };
    std::function<size_t(TreeNode *)> treesize = [&](TreeNode *r) -> size_t {
        if (!r) {
            return 0;
        }
        return treesize(r->left) + 1 + treesize(r->right);
    };
    size_t r1_len = treesize(root1);
    size_t r2_len = treesize(root2);

    vector<int> traverse1, traverse2;
    traverse1.reserve(r1_len);
    traverse2.reserve(r2_len);
    in_order_traverse(root1, traverse1);
    in_order_traverse(root2, traverse2);
    vector<int> ans;
    ans.reserve(traverse1.size() + traverse2.size());
    std::merge(traverse1.begin(), traverse1.end(), traverse2.begin(),
               traverse2.end(), std::back_inserter(ans));
    return ans;
}

typedef vector<int>::iterator Iter;

int helper(vector<int> &nums, Iter lstart, Iter lend, Iter rstart, Iter rend,
           int val, int mask) {
    if (lstart == lend || rstart == rend || mask == 0)
        return val;
    auto cond = [mask](int num) { return num & mask; };
    auto lmid = partition(lstart, lend, cond),
         rmid = partition(rstart, rend, cond);
    if (lmid == lend && rmid == rend || lmid == lstart && rmid == rstart)
        return helper(nums, lstart, lend, rstart, rend, val, mask >> 1);
    else
        return max(
            helper(nums, lstart, lmid, rmid, rend, val | mask, mask >> 1),
            rstart < lend ? 0
                          : helper(nums, lmid, lend, rstart, rmid, val | mask,
                                   mask >> 1));
}
int Solution::findMaximumXOR(vector<int> &nums) {
    return helper(nums, nums.begin(), nums.end(), nums.begin(), nums.end(), 0,
                  1 << 30);
}

int Solution::largestRectangleArea(vector<int> &heights) {
    int largest = 0;
    stack<size_t> stack;
    size_t idx = 0;
    while (idx < heights.size()) {
        if (stack.empty() || heights[stack.top()] <= heights[idx]) {
            stack.push(idx);
            idx++;
            continue;
        }
        size_t current = stack.top();
        stack.pop();
        int width = 0;
        if (stack.empty()) {
            width = idx;
        } else {
            width = idx - stack.top() - 1;
        }
        largest = std::max(largest, width * heights[current]);
    }
    while (!stack.empty()) {
        int width = 0;
        size_t cur = stack.top();
        stack.pop();
        if (stack.empty()) {
            width = idx;
        } else {
            width = idx - stack.top() - 1;
        }
        largest = std::max(largest, width * heights[cur]);
    }

    return largest;
}

void Solution::rotate(vector<int> &nums, int k) {
    k = k % nums.size();
    std::rotate(nums.begin(), nums.begin() + nums.size() - k, nums.end());
}

int Solution::maximumWealth(vector<vector<int>> &accounts) {
    int max = 0;
    for (const auto &a : accounts) {
        max = std::max(std::accumulate(a.begin(), a.end(), 0), max);
    }
    return max;
}