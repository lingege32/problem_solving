#include <vector>
using namespace std;
class Solution {
public:
    bool isArraySpecial(vector<int>& nums) {
        bool tag = 0==(nums[0]%2);
        for(size_t idx=1; idx<nums.size(); ++idx) {
            if ((tag && nums[idx]%2==0) || (!tag && nums[idx]%2==1)) {
                return false;
            } 
            tag = !tag;
        }
        return true;
    }
};