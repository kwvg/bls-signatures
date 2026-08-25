// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#include "secure.h"
#include "util.hpp"

#include <utility>

namespace bls {
namespace util {
Bn::Bn()
{
    bn_null(m_val);
    bn_new(m_val);
    bn_zero(m_val);
}

Bn::~Bn()
{
    SecureWipe(m_val, sizeof(bn_st));
    bn_free(m_val);
}

Bn::Bn(Bn&& other) noexcept : Bn() { *this = std::move(other); }

Bn& Bn::operator=(Bn&& other) noexcept
{
    if (this != &other) {
        SecureWipe(m_val->dp, sizeof(m_val->dp));
        bn_copy(m_val, other.m_val);
        SecureWipe(other.m_val, sizeof(bn_st));
        bn_new(other.m_val);
        bn_zero(other.m_val);
    }
    return *this;
}
} // namespace util
} // namespace bls
