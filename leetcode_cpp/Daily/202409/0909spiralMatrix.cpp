
struct ListNode {
    int val;
    ListNode *next;

    ListNode():
      val(0),
      next(nullptr) {}

    explicit ListNode(int x):
      val(x),
      next(nullptr) {}

    ListNode(int x, ListNode *next):
      val(x),
      next(next) {}
};

#include <bits/stdc++.h>
using namespace std;

class Solution {
    enum class Dir { Right, Down, Left, Up };

    static Dir changeDir(Dir d) { return static_cast<Dir>((static_cast<int>(d) + 1) & 0x3); }

  public:
    static vector<vector<int>> spiralMatrix(int m, int n, ListNode *head) {
        std::vector<std::vector<int>> res(m, std::vector<int>(n, -1));
        int i = 0, j = 0;
        Dir dir = Dir::Right;
        auto move = [&]() {
            int ni = i;
            int nj = j;
            switch (dir) {
                case Dir::Right:
                    ++nj;
                    break;
                case Dir::Down:
                    ++ni;
                    break;
                case Dir::Left:
                    --nj;
                    break;
                case Dir::Up:
                    --ni;
                    break;
            }
            if (ni < 0 || nj < 0 || nj >= n || ni >= m || res[ni][nj] != -1) {
                dir = changeDir(dir);
                switch (dir) {
                    case Dir::Right:
                        ++j;
                        break;
                    case Dir::Down:
                        ++i;
                        break;
                    case Dir::Left:
                        --j;
                        break;
                    case Dir::Up:
                        --i;
                        break;
                }
            } else {
                i = ni;
                j = nj;
            }
        };
        while (head) {
            res[i][j] = head->val;
            move();
            head = head->next;
        }
        return res;
    }
};