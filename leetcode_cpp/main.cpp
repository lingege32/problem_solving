#include "header1.h"
#include <cassert>

int main() {
    Solution s;
    assert(s.bitwiseComplement(5) == 2);
    assert(s.bitwiseComplement(7) == 0);
    assert(s.bitwiseComplement(10) == 5);

    vector<vector<string>> a1 = {{"a", "a", "b"}, {"aa", "b"}};
    assert(s.partition("aab") == a1);
    vector<vector<string>> a2 = {{"a"}};
    assert(s.partition("a") == a2);
    vector<vector<string>> a3 = {{"b", "b"}, {"bb"}};
    assert(s.partition("bb") == a3);
    vector<vector<string>> a4 = {{"e", "f", "e"}, {"efe"}};
    assert(s.partition("efe") == a4);
    vector<vector<string>> a5 = {{"c", "b", "b", "b", "c", "c"},
                                 {"c", "b", "b", "b", "cc"},
                                 {"c", "b", "bb", "c", "c"},
                                 {"c", "b", "bb", "cc"},
                                 {"c", "bb", "b", "c", "c"},
                                 {"c", "bb", "b", "cc"},
                                 {"c", "bbb", "c", "c"},
                                 {"c", "bbb", "cc"},
                                 {"cbbbc", "c"}};
    assert(s.partition("cbbbcc") == a5);

    std::cout << s.multiply("2", "3") << "??" << std::endl;
    ;

    Solution swildcard;
    std::cout<<swildcard.isWildCardMatch("abefcdgiescdfimde", "ab*cd?i*de")<<"<<"<<std::endl;;
}
