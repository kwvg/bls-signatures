// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#ifndef DASHBLS_SECURE_H
#define DASHBLS_SECURE_H

#include "wipe.h"

#include "relic_conf.h"

#if defined GMP && ARITH == GMP
#include <gmp.h>
#endif

extern "C" {
#include "relic.h"
}

#include <cstddef>

// BLS::Init refuses to run unless relic was built ALLOC=AUTO, which puts
// bn_st's digits inline rather than behind a pointer. That inline storage is
// what gets wiped, so catch a mismatch here rather than wipe the wrong bytes.
#if !defined(ALLOC) || !defined(AUTO) || ALLOC != AUTO
#error "secure.h assumes relic is built with ALLOC == AUTO"
#endif

namespace bls {
namespace util {
/**
 * An owning relic bn_t that clears itself when destroyed.
 *
 * Constructing one initialises it, which relic requires before any use.
 * Destroying one clears the whole bn_st, so a value that was a secret does not
 * outlive its scope, and the used and sign fields do not survive either.
 */
class Bn {
public:
    Bn();
    ~Bn();

    Bn(const Bn&) = delete;
    Bn& operator=(const Bn&) = delete;

    /**
     * Takes over another's value, leaving it cleared and zero.
     *
     * Move-only, so a secret is never silently duplicated. The digits are
     * inline under ALLOC=AUTO, so there is no buffer to steal, and the value is
     * copied and the source cleared.
     */
    Bn(Bn&& other) noexcept;
    Bn& operator=(Bn&& other) noexcept;

    /**
     * Converts to the underlying bn_t.
     *
     * @returns A reference that relic entry points taking a bn_t accept.
     */
    operator bn_t&() { return m_val; }
    operator const bn_t&() const { return m_val; }

    /**
     * Names the conversion, for where a bn_t is not deduced.
     *
     * @returns A reference to the underlying bn_t.
     */
    bn_t& native() { return m_val; }
    const bn_t& native() const { return m_val; }

private:
    bn_t m_val;
};
} // namespace util
} // namespace bls

#endif // DASHBLS_SECURE_H
