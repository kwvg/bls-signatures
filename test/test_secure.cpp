// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#include "bls.hpp"
#include "secure.h"

#include <catch2/catch.hpp>

#include <algorithm>
#include <array>
#include <cstdlib>
#include <cstring>
#include <limits>
#include <memory>
#include <new>
#include <thread>
#include <utility>
#include <vector>

namespace {
alignas(std::max_align_t) uint8_t g_arena[4096];
size_t g_arenaUsed{0};

void* ArenaAlloc(size_t nBytes)
{
    constexpr size_t nAlign = alignof(std::max_align_t);
    const size_t nStep = (nBytes + nAlign - 1) & ~(nAlign - 1);
    if (nStep > sizeof(g_arena) - g_arenaUsed) {
        return nullptr;
    }
    void* pBlock = g_arena + g_arenaUsed;
    g_arenaUsed += nStep;
    return pBlock;
}

void ArenaFree(void*) {}

struct ArenaGuard {
    ArenaGuard()
    {
        bls::util::GetSecureAllocator(&m_pfnAlloc, &m_pfnFree);
        g_arenaUsed = 0;
        bls::util::SetSecureAllocator(ArenaAlloc, ArenaFree);
    }

    ~ArenaGuard()
    {
        if (m_pfnAlloc != nullptr && m_pfnFree != nullptr) {
            bls::util::SetSecureAllocator(m_pfnAlloc, m_pfnFree);
        }
    }

private:
    bls::Util::SecureAllocCallback m_pfnAlloc{nullptr};
    bls::Util::SecureFreeCallback m_pfnFree{nullptr};
};
} // anonymous namespace

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

TEST_CASE("Secure allocation is safe to use from several threads")
{
    // The callbacks are reached from every thread that holds a key, so this is
    // here to be run under a sanitizer rather than to assert an outcome.
    std::vector<std::thread> threads;

    for (int i = 0; i < 4; ++i) {
        threads.emplace_back([] {
            for (int j = 0; j < 250; ++j) {
                auto* p = bls::util::SecAlloc<uint8_t>(64);
                std::memset(p, 0x7e, 64);
                bls::util::SecFree(p, 64);
            }
        });
    }
    for (auto& thread : threads) {
        thread.join();
    }
    SUCCEED("no allocation raced another");
}

TEST_CASE("The secure allocator is never called through a null")
{
    REQUIRE_THROWS_AS(bls::util::SetSecureAllocator(nullptr, std::free),
                      std::invalid_argument);
    REQUIRE_THROWS_AS(bls::util::SetSecureAllocator(std::malloc, nullptr),
                      std::invalid_argument);

    // Rejecting the bad pair must not have disturbed the installed one.
    auto* p = bls::util::SecAlloc<uint8_t>(32);
    REQUIRE(p != nullptr);
    bls::util::SecFree(p, 32);
}

TEST_CASE("Secure allocation failure throws rather than returning null")
{
    // The raw SecAlloc callers do not check for null, so a count that does not
    // fit in a size_t once scaled by the object size has to throw rather than
    // wrap into a small allocation the caller then writes past.
    REQUIRE_THROWS_AS(bls::util::SecAlloc<uint64_t>(
        std::numeric_limits<size_t>::max() / 4), std::bad_alloc);
    REQUIRE_THROWS_AS(bls::util::SecAlloc<uint32_t>(
        std::numeric_limits<size_t>::max()), std::bad_alloc);
}

TEST_CASE("A private key has value semantics and clears itself")
{
    SECTION("Destruction leaves nothing behind")
    {
        // The key's bn_st goes back through SecFree, which the installed
        // allocator need not clear, so the extent has to travel with it.
        std::memset(g_arena, 0xcd, sizeof(g_arena));
        {
            ArenaGuard guard;
            bls::PrivateKey sk = bls::PrivateKey::RandomPrivateKey();
            REQUIRE(std::any_of(g_arena, g_arena + sizeof(bn_st),
                                [](uint8_t b) { return b != 0 && b != 0xcd; }));
        }

        REQUIRE(std::all_of(g_arena, g_arena + sizeof(bn_st),
                            [](uint8_t b) { return b == 0; }));
    }
}
