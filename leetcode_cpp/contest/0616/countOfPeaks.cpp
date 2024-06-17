#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

struct SegTree {
    size_t n;
    vector<int> tree;
    explicit SegTree(size_t n) : n{n} { tree.resize(4 * n, 0); }
    void update(size_t idx, int val) {
        update_internal(0, val, idx, 0, n - 1);
    }
    int update_internal(size_t tree_idx, int val, size_t target_idx, size_t cur_left, size_t cur_right) {
        if (cur_left == cur_right && target_idx == cur_left) {
            auto ori = tree[tree_idx];
            auto modify = val - ori;
            tree[tree_idx] = val;
            return modify;
        }
        auto mid = (cur_left + cur_right) / 2;
        auto modify = [&]() {
            if (target_idx <= mid) {
                return update_internal(2 * tree_idx + 1, val, target_idx, cur_left, mid);
            }
            return update_internal(2 * tree_idx + 2, val, target_idx, mid+1, cur_right);
        }();
        tree[tree_idx] += modify;
        return modify;
    }
    int query(size_t left, size_t right) {
        if (right - left < 2) {
            return 0;
        }
        return query_internal(0, left + 1, right - 1, 0, n - 1);
    }
    int query_internal(size_t tree_idx, size_t left, size_t right,
                       size_t cur_left, size_t cur_right) {
        if (left == cur_left && right == cur_right) {
            return tree[tree_idx];
        }
        auto mid = (cur_left + cur_right) / 2;
        if (right <= mid) {
            return query_internal(2 * tree_idx + 1, left, right, cur_left, mid);
        }
        if (left > mid) {
            return query_internal(2 * tree_idx + 2, left, right, mid + 1,
                                  cur_right);
        }
        return query_internal(2 * tree_idx + 1, left, mid, cur_left, mid) +
               query_internal(2 * tree_idx + 2, mid + 1, right, mid + 1,
                              cur_right);
    }
    void print(size_t idx, size_t left, size_t right) {
        std::cout << "(," << left << ", " << right << ") = " << tree[idx]
                  << "\n";
        if (left == right) {
            return;
        }
        auto mid = (left + right) / 2;
        print(2 * idx + 1, left, mid);
        print(2 * idx + 2, mid + 1, right);
    }
};
class Solution {
  public:
    vector<int> countOfPeaks(vector<int> &nums, vector<vector<int>> &queries) {
        size_t s = nums.size();
        SegTree tree{s};
        for (size_t i = 1; i < s - 1; ++i) {
            if (nums[i] > nums[i - 1] && nums[i] > nums[i + 1]) {
                tree.update(i, 1);
            }
        }
        
        tree.print(0, 0, s - 1);
        auto ret = prepare(queries);
        for (const auto &query : queries) {
            auto a = query[0];
            auto b = query[1];
            auto c = query[2];
            if (a == 1) {
                ret.push_back(tree.query(b, c));
            } else {
                nums[b] = c;
                auto l = std::max(1, b - 1);
                auto r = std::min(static_cast<int>(s) - 2, b + 1);
                for (; l <= r; ++l) {
                    if (nums[l] > nums[l - 1] && nums[l] > nums[l + 1]) {
                        tree.update(l, 1);
                    } else {
                        tree.update(l, 0);
                    }
                }
            }
        }
        tree.print(0, 0, s - 1);
        return ret;
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
};

// class Solution2 {
//   public:
//     static bool isPeak(const vector<int> &nums, int i) {
//         if (i <= 0 || i >= nums.size() - 1)
//             return false;
//         return nums[i] > nums[i - 1] && nums[i] > nums[i + 1];
//     }

//     class SegmentTree {
//       private:
//         vector<int> tree;
//         int n;

//         void build(const vector<int> &nums, int node, int start, int end) {
//             if (start == end) {
//                 tree[node] = isPeak(nums, start) ? 1 : 0;
//             } else {
//                 int mid = (start + end) / 2;
//                 build(nums, 2 * node + 1, start, mid);
//                 build(nums, 2 * node + 2, mid + 1, end);
//                 tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
//             }
//         }

//         void update(const vector<int> &nums, int node, int start, int end,
//                     int idx) {
//             if (start == end) {
//                 tree[node] = isPeak(nums, idx) ? 1 : 0;
//             } else {
//                 int mid = (start + end) / 2;
//                 if (start <= idx && idx <= mid) {
//                     update(nums, 2 * node + 1, start, mid, idx);
//                 } else {
//                     update(nums, 2 * node + 2, mid + 1, end, idx);
//                 }
//                 tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
//             }
//         }

//         int query(int node, int start, int end, int l, int r) {
//             if (r < start || end < l) {
//                 return 0;
//             }
//             if (l <= start && end <= r) {
//                 return tree[node];
//             }
//             int mid = (start + end) / 2;
//             int left_query = query(2 * node + 1, start, mid, l, r);
//             int right_query = query(2 * node + 2, mid + 1, end, l, r);
//             return left_query + right_query;
//         }

//       public:
//         SegmentTree(const vector<int> &nums) {
//             n = nums.size();
//             tree.resize(4 * n);
//             build(nums, 0, 0, n - 1);
//         }

//         void update(const vector<int> &nums, int idx) {
//             update(nums, 0, 0, n - 1, idx);
//         }

//         int query(int l, int r) { return query(0, 0, n - 1, l, r); }
//     };

//     vector<int> countOfPeaks(vector<int> &nums, vector<vector<int>> &queries)
//     {
//         int n = nums.size();
//         vector<int> results;
//         SegmentTree segTree(nums);

//         for (const auto &query : queries) {
//             if (query[0] == 1) {
//                 int l = query[1];
//                 int r = query[2];
//                 results.push_back(segTree.query(l + 1, r - 1));
//             } else if (query[0] == 2) {
//                 int index = query[1];
//                 int val = query[2];
//                 nums[index] = val;
//                 if (index > 0)
//                     segTree.update(nums, index - 1);
//                 segTree.update(nums, index);
//                 if (index < n - 1)
//                     segTree.update(nums, index + 1);
//             }
//         }

//         return results;
//     }
// };