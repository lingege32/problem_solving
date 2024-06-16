#include <algorithm>
#include <vector>
using namespace std;

class Solution {
  public:
    vector<int> countOfPeaks(vector<int> &nums, vector<vector<int>> &queries) {
        auto ret = prepare(queries);
        auto table = prepare_table(nums);
        for (const auto &query : queries) {
            auto a = query[0];
            auto b = query[1];
            auto c = query[2];
            if (a == 1) {
                ret.push_back(check(table, b, c));
            } else {
                // for (auto t : table) {
                //     std::cout << static_cast<int>(t) << " ";
                // }
                // std::cout << "\n";
                change(table, nums, b, c);
                // for (auto t : table) {
                //     std::cout << static_cast<int>(t) << " ";
                // }
                // std::cout << "\n";
            }
        }
        return ret;
    }

    static void change(vector<char> &table, vector<int> &nums, size_t idx,
                       int val) {
        if (idx != 0) {
            auto l = nums[idx - 1];
            if (val > l) {
                table[idx] |= 1;
                table[idx - 1] &= 1;
            } else if (l > val) {
                table[idx] &= 2;
                table[idx - 1] |= 2;
            } else {
                table[idx] &= 2;
                table[idx - 1] &= 1;
            }
        }
        if (idx != nums.size() - 1) {
            auto r = nums[idx + 1];
            if (val > r) {
                table[idx] |= 2;
                table[idx + 1] &= 2;
            } else if (val < r) {
                table[idx] &= 1;
                table[idx + 1] |= 1;

            } else {
                table[idx] &= 1;
                table[idx + 1] &= 2;
            }
        }
        nums[idx] = val;
    }
    static int check(vector<char> &table, size_t left, size_t right) {
        left += 1;
        if (left >= right) {
            return 0;
        }
        return std::count(table.begin() + left, table.begin() + right, 3);
    }

    static vector<int> prepare(const vector<vector<int>> &q) {
        vector<int> ret;
        size_t c = 0;
        for (auto &qq : q) {
            if (qq[0] == 1) {
                c++;
            }
        }
        ret.reserve(c);
        return ret;
    }
    static vector<char> prepare_table(const vector<int> &nums) {
        vector<char> table;
        table.resize(nums.size(), 0);
        for (auto idx = 1U; idx < nums.size() - 1; ++idx) {
            if (nums[idx] > nums[idx - 1]) {
                table[idx] |= 1;
            }
            if (nums[idx] > nums[idx + 1]) {
                table[idx] |= 2;
            }
        }
        if (nums[0] > nums[1]) {
            table[0] |= 2;
        }
        auto s = table.size();
        if (nums[s - 1] > nums[s - 2]) {
            table[s - 1] |= 1;
        }
        return table;
    }
};

class Solution2 {
public:

    static bool isPeak(const vector<int>& nums, int i) {
        if (i <= 0 || i >= nums.size() - 1) return false;
        return nums[i] > nums[i-1] && nums[i] > nums[i+1];
    }


    class SegmentTree {
    private:
        vector<int> tree;
        int n;

        void build(const vector<int>& nums, int node, int start, int end) {
            if (start == end) {
                tree[node] = isPeak(nums, start) ? 1 : 0;
            } else {
                int mid = (start + end) / 2;
                build(nums, 2 * node + 1, start, mid);
                build(nums, 2 * node + 2, mid + 1, end);
                tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
            }
        }

        void update(const vector<int>& nums, int node, int start, int end, int idx) {
            if (start == end) {
                tree[node] = isPeak(nums, idx) ? 1 : 0;
            } else {
                int mid = (start + end) / 2;
                if (start <= idx && idx <= mid) {
                    update(nums, 2 * node + 1, start, mid, idx);
                } else {
                    update(nums, 2 * node + 2, mid + 1, end, idx);
                }
                tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
            }
        }

        int query(int node, int start, int end, int l, int r) {
            if (r < start || end < l) {
                return 0;
            }
            if (l <= start && end <= r) {
                return tree[node];
            }
            int mid = (start + end) / 2;
            int left_query = query(2 * node + 1, start, mid, l, r);
            int right_query = query(2 * node + 2, mid + 1, end, l, r);
            return left_query + right_query;
        }

    public:
        SegmentTree(const vector<int>& nums) {
            n = nums.size();
            tree.resize(4 * n);
            build(nums, 0, 0, n - 1);
        }

        void update(const vector<int>& nums, int idx) {
            update(nums, 0, 0, n - 1, idx);
        }

        int query(int l, int r) {
            return query(0, 0, n - 1, l, r);
        }
    };

    vector<int> countOfPeaks(vector<int>& nums, vector<vector<int>>& queries) {
        int n = nums.size();
        vector<int> results;
        SegmentTree segTree(nums);

        for (const auto& query : queries) {
            if (query[0] == 1) {
                int l = query[1];
                int r = query[2];
                results.push_back(segTree.query(l + 1, r - 1));
            } else if (query[0] == 2) {
                int index = query[1];
                int val = query[2];
                nums[index] = val;
                if (index > 0) segTree.update(nums, index - 1);
                segTree.update(nums, index);
                if (index < n - 1) segTree.update(nums, index + 1);
            }
        }

        return results;
    }
};