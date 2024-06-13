#include <string>
using namespace std;
class Solution {
public:
    int minimumChairs(string s) {
        int max = 0;
        int ans = 0;
        for (const auto c: s) {
            if (c=='E') {
                ans++;
                max=std::max(max,ans);
            } else {
                ans--;
            }
        }
        return max;
    }
};