// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#include "secure.h"

#include <catch2/catch.hpp>

#include <algorithm>
#include <array>
#include <new>
#include <utility>
#include <vector>

TEST_CASE("util::SecureWipe clears a buffer")
{
    std::array<uint8_t, 32> buf;
    buf.fill(0xff);
    bls::util::SecureWipe(buf.data(), buf.size());
    REQUIRE(std::all_of(buf.begin(), buf.end(), [](uint8_t b) {
        return b == 0;
    }));

    // Degenerate arguments are ignored rather than faulting.
    bls::util::SecureWipe(nullptr, 16);
    bls::util::SecureWipe(buf.data(), 0);
}

TEST_CASE("A bignum has value semantics and clears itself")
{
    SECTION("A move takes the value and leaves the source cleared")
    {
        bls::util::Bn expected;
        g1_get_ord(expected);
        bls::util::Bn order;
        g1_get_ord(order);
        REQUIRE(bn_is_zero(order) == 0);

        bls::util::Bn moved{std::move(order)};
        REQUIRE(bn_cmp(moved, expected) == RLC_EQ);
        REQUIRE(bn_is_zero(order) == 1);

        bls::util::Bn assigned;
        assigned = std::move(moved);
        REQUIRE(bn_cmp(assigned, expected) == RLC_EQ);
        REQUIRE(bn_is_zero(moved) == 1);

        // Self-move is a no-op rather than a clear.
        auto& alias = assigned;
        assigned = std::move(alias);
        REQUIRE(bn_cmp(assigned, expected) == RLC_EQ);
    }

    SECTION("Default construction leaves a usable, zeroed value")
    {
        std::vector<bls::util::Bn> vec(4);
        REQUIRE(bn_is_zero(vec[3]) == 1);
        bn_set_dig(vec[3], 4);
        REQUIRE(bn_cmp_dig(vec[3], 4) == RLC_EQ);
    }

    SECTION("Destruction leaves nothing behind")
    {
        alignas(bn_st) std::array<uint8_t, sizeof(bls::util::Bn)> storage;
        storage.fill(0xa5);

        auto* secret = new (storage.data()) bls::util::Bn();
        bn_set_dig(secret->native(), 42);
        REQUIRE(bn_cmp_dig(secret->native(), 42) == RLC_EQ);
        secret->~Bn();

        REQUIRE(std::all_of(storage.begin(), storage.end(), [](uint8_t b) {
            return b == 0;
        }));
    }
}
