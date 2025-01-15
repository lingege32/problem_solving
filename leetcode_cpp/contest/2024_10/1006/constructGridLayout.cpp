#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static vector<vector<int>> constructGridLayout(int n, vector<vector<int>>& edges);
};

struct node_t {
    uint64_t m_uValue;

    bool operator<(const node_t& other) const { return m_uValue < other.m_uValue; }

    node_t& operator=(uint64_t uValue) {
        m_uValue = uValue;
        return *this;
    }

    [[nodiscard]] unsigned degree() const { return (std::bit_width(m_uValue) + 15U) / 16U; }

    [[nodiscard]] uint16_t get_edge(unsigned uEdge_i) const {
        return static_cast<uint16_t>(m_uValue >> (uEdge_i * 16U));
    }

    void add_edge(uint16_t uEdge) { m_uValue = (m_uValue << 16) + uEdge; }
};

std::vector<std::vector<int>> Solution::constructGridLayout(  // NOLINT(readability-function-cognitive-complexity)
    int n, std::vector<std::vector<int>>& edges) {
    size_t const uN = n;
    std::vector<node_t> vNode(uN);
    for (std::vector<int> const& edge : edges) {
        node_t& stNode0 = vNode[(size_t)edge[0]];
        node_t& stNode1 = vNode[(size_t)edge[1]];

        stNode0.add_edge((uint16_t)(edge[1] + 1));
        stNode1.add_edge((uint16_t)(edge[0] + 1));
    }
    node_t const* pNode_deg1 = nullptr;
    node_t const* pNode_deg2 = nullptr;
    for (node_t const& stNode : vNode) {
        unsigned const uDeg = stNode.degree();
        if (uDeg == 1U) {
            pNode_deg1 = std::addressof(stNode);
            break;
        }
        if (uDeg == 2U) {
            pNode_deg2 = std::addressof(stNode);
        } else if (pNode_deg2) {
            break;
        }
    }

    if (pNode_deg1) {
        std::vector<std::vector<int>> vvIndex(1);
        std::vector<int>& vIndex = vvIndex[0];
        vIndex.reserve(uN);
        vIndex.push_back((int)(pNode_deg1 - vNode.data()));
        for (uint16_t uIndex_prev = (uint16_t)(pNode_deg1 - vNode.data()),
                      uIndex = (uint16_t)(vNode[uIndex_prev].get_edge(0) - 1U), uCol = 1U;
             uCol < uN; ++uCol) {
            vIndex.push_back(uIndex);
            uIndex_prev = std::exchange(
                uIndex, uint16_t(((uIndex_prev + 1U) ^ vNode[uIndex].get_edge(0) ^ vNode[uIndex].get_edge(1)) - 1U));
        }
        return vvIndex;
    }

    std::vector<std::vector<int>> vvIndex;
    auto const lambdaNextIndex_deg3 = [&](uint16_t const uIndex, uint16_t const uIndex_prev) -> uint16_t {
        auto tupleEdge = std::make_tuple(vNode[uIndex].get_edge(0), vNode[uIndex].get_edge(1),
                                         vNode[uIndex].get_edge(2));
        if (std::get<0>(tupleEdge) == uIndex_prev + 1U) {
            std::swap(std::get<0>(tupleEdge), std::get<2>(tupleEdge));
        }
        if (std::get<1>(tupleEdge) == uIndex_prev + 1U) {
            std::swap(std::get<1>(tupleEdge), std::get<2>(tupleEdge));
        }
        if (std::get<0>(tupleEdge) && std::get<1>(tupleEdge)) {
            if (vNode[std::get<0>(tupleEdge) - 1U] < vNode[std::get<1>(tupleEdge) - 1U]) {
                return std::get<0>(tupleEdge) - 1U;
            }
            return std::get<1>(tupleEdge) - 1U;
        }
        if (std::get<0>(tupleEdge)) {
            return std::get<0>(tupleEdge) - 1U;
        }
        if (std::get<1>(tupleEdge)) {
            return std::get<1>(tupleEdge) - 1U;
        }
        std::abort();
    };
    auto const lambdaNextIndex_deg4 = [&](uint16_t const uIndex_00, uint16_t const uIndex_01, uint16_t const uIndex_10,
                                          uint16_t const uIndex_10_prev) -> uint16_t {
        node_t const& stNode_01 = vNode[uIndex_01];
        node_t const& stNode_10 = vNode[uIndex_10];
        auto const tupleEdge_01 = std::make_tuple(stNode_01.get_edge(0), stNode_01.get_edge(1), stNode_01.get_edge(2),
                                                  stNode_01.get_edge(3));
        auto const tupleEdge_10 = std::make_tuple(stNode_10.get_edge(0), stNode_10.get_edge(1), stNode_10.get_edge(2),
                                                  stNode_10.get_edge(3));
        for (uint16_t const uEdge : {std::get<0>(tupleEdge_10), std::get<1>(tupleEdge_10), std::get<2>(tupleEdge_10),
                                     std::get<3>(tupleEdge_10)}) {
            uint16_t const uIndex = uEdge - 1U;
            if (uIndex != uIndex_00 && uIndex != uIndex_10_prev) {
                if (uEdge == std::get<0>(tupleEdge_01) || uEdge == std::get<1>(tupleEdge_01) ||
                    uEdge == std::get<2>(tupleEdge_01) || uEdge == std::get<3>(tupleEdge_01)) {
                    return uIndex;
                }
            }
        }
        std::abort();
    };
    uint16_t const uIndex_corner = pNode_deg2 - vNode.data();
    uint16_t uIndex_dir0 = vNode[uIndex_corner].get_edge(0) - 1U;
    uint16_t uIndex_dir1 = vNode[uIndex_corner].get_edge(1) - 1U;
    uint16_t uLength_shorter = 2U;
    for (uint16_t uIndex_prev0 = uIndex_corner, uIndex_prev1 = uIndex_corner;
         vNode[uIndex_dir0].get_edge(2) && vNode[uIndex_dir1].get_edge(2); ++uLength_shorter) {
        uIndex_dir0 = lambdaNextIndex_deg3(uIndex_dir0, std::exchange(uIndex_prev0, uIndex_dir0));
        uIndex_dir1 = lambdaNextIndex_deg3(uIndex_dir1, std::exchange(uIndex_prev1, uIndex_dir1));
    }
    uint16_t const uLength_longer = n / uLength_shorter;
    vvIndex.resize(uLength_shorter, std::vector<int>(uLength_longer));
    if (uLength_shorter == 2U) {
        std::vector<int>& vIndex0 = vvIndex[0];
        std::vector<int>& vIndex1 = vvIndex[1];
        vIndex0[0] = uIndex_corner;
        vIndex1[0] = vNode[uIndex_corner].get_edge(vNode[uIndex_dir0] < vNode[uIndex_dir1] ? 0 : 1) - 1U;
        for (uint16_t
                 uIndex0_prev = vIndex0[0],
                 uIndex1_prev = vIndex1[0],
                 uIndex0 = uint16_t(
                     ((uIndex1_prev + 1U) ^ vNode[uIndex0_prev].get_edge(0) ^ vNode[uIndex0_prev].get_edge(1)) - 1U),
                 uIndex1 = uint16_t(
                     ((uIndex0_prev + 1U) ^ vNode[uIndex1_prev].get_edge(0) ^ vNode[uIndex1_prev].get_edge(1)) - 1U),
                 uCol = 1;
             uCol < uLength_longer; ++uCol) {
            vIndex0[uCol] = uIndex0;
            vIndex1[uCol] = uIndex1;
            uint16_t const uIndex0_next = (vNode[uIndex0].get_edge(0) ^ vNode[uIndex0].get_edge(1) ^
                                           vNode[uIndex0].get_edge(2) ^ (uIndex1 + 1U) ^ (uIndex0_prev + 1U)) -
                                          1U;
            uint16_t const uIndex1_next = (vNode[uIndex1].get_edge(0) ^ vNode[uIndex1].get_edge(1) ^
                                           vNode[uIndex1].get_edge(2) ^ (uIndex0 + 1U) ^ (uIndex1_prev + 1U)) -
                                          1U;
            uIndex0_prev = uIndex0;
            uIndex1_prev = uIndex1;
            uIndex0 = uIndex0_next;
            uIndex1 = uIndex1_next;
        }
    } else {
        {
            std::vector<int>& vIndex = vvIndex[0];
            vIndex[0] = uIndex_corner;
            for (uint16_t uIndex_prev = uIndex_corner,
                          uIndex = vNode[uIndex_corner].get_edge(vNode[uIndex_dir0] < vNode[uIndex_dir1] ? 1 : 0) - 1U,
                          uCol = 1;
                 uCol < uLength_longer; ++uCol) {
                vIndex[uCol] = uIndex;

                uIndex_prev = std::exchange(uIndex, lambdaNextIndex_deg3(uIndex, uIndex_prev));
            }
        }
        for (uint16_t row = 1; row < uLength_shorter; ++row) {
            std::vector<int> const& vIndex0 = vvIndex[row - 1U];
            std::vector<int>& vIndex1 = vvIndex[row];
            vIndex1[0] = (vNode[vIndex0[0]].get_edge(0) ^ vNode[vIndex0[0]].get_edge(1) ^
                          vNode[vIndex0[0]].get_edge(2) ^ (vIndex0[1] + 1U) ^
                          (row >= 2U ? vvIndex[row - 2U][0] + 1U : 0U)) -
                         1U;
            for (uint16_t uCol = 1, uIndex1_i = 0xffffU, uIndex1_j = vIndex1[0]; uCol < uLength_longer; ++uCol) {
                uint16_t const uIndex1 = lambdaNextIndex_deg4(vIndex0[uCol - 1], vIndex0[uCol], uIndex1_j, uIndex1_i);

                vIndex1[uCol] = uIndex1;
                uIndex1_i = uIndex1_j;
                uIndex1_j = uIndex1;
            }
        }
    }
    return vvIndex;

    std::abort();
}
