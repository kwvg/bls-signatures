// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#ifndef DASHBLS_SECURE_H
#define DASHBLS_SECURE_H

#include "relic_conf.h"

#if defined GMP && ARITH == GMP
#include <gmp.h>
#endif

extern "C" {
#include "relic.h"
}

#include <cstddef>

namespace bls {
/**
 * RAII wrapper for a dynamically allocated array of bn_t.
 *
 * Ensures bn_free and delete[] run even when an exception is thrown, and
 * tracks how many were initialised so partial construction still cleans up.
 */
struct BnArrayGuard {
    bn_t* data;
    size_t count;
    size_t initialized{0};

    explicit BnArrayGuard(size_t n);
    ~BnArrayGuard();

    bn_t& operator[](size_t i) { return data[i]; }
    const bn_t& operator[](size_t i) const { return data[i]; }

    BnArrayGuard(const BnArrayGuard&) = delete;
    BnArrayGuard& operator=(const BnArrayGuard&) = delete;
};

/**
 * RAII wrapper for a single bn_t value.
 */
struct BnGuard {
    bn_t val;

    BnGuard();
    ~BnGuard();

    operator bn_t&() { return val; }

    BnGuard(const BnGuard&) = delete;
    BnGuard& operator=(const BnGuard&) = delete;
};
} // namespace bls

#endif // DASHBLS_SECURE_H
