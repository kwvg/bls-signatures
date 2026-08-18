// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#include "secure.h"

namespace bls {
BnArrayGuard::BnArrayGuard(size_t n) : data(new bn_t[n]), count(n)
{
    for (size_t i = 0; i < count; i++) {
        bn_new(data[i]);
        initialized++;
    }
}

BnArrayGuard::~BnArrayGuard()
{
    for (size_t i = 0; i < initialized; i++) {
        bn_free(data[i]);
    }
    delete[] data;
}

BnGuard::BnGuard() { bn_new(val); }

BnGuard::~BnGuard() { bn_free(val); }
} // namespace bls
