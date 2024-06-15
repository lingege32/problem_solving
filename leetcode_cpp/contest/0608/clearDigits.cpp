#include <string>
#include <stack>
using namespace std;


class Solution {
public:
    string clearDigits(string s) {
        std::stack<size_t> charater_idx;
        for (size_t idx=0; idx<s.size(); ++idx) {
            auto& c = s[idx];
            if (c>='a') {
                charater_idx.push(idx);
            } else if (c>='0') {
                c='.';
                auto top = charater_idx.top();
                s[top] = '.';
                charater_idx.pop();
            }
        }
        std::string ret;
        for(auto c: s) {
            if (c!='.') {
                ret.push_back(c);
            }
        }
        return ret;
        
    }
};