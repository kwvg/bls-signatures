// Copyright (c) 2026 The Dash Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING.MIT or https://opensource.org/license/MIT

#include "hdkeys.hpp"

namespace bls {
PrivateKey HDKeys::KeyGen(const std::vector<uint8_t>& seed)
{
    return KeyGen(Bytes(seed));
}

PrivateKey HDKeys::KeyGen(const Bytes& seed)
{
    // KeyGen
    // 1. PRK = HKDF-Extract("BLS-SIG-KEYGEN-SALT-", IKM || I2OSP(0, 1))
    // 2. OKM = HKDF-Expand(PRK, keyInfo || I2OSP(L, 2), L)
    // 3. SK = OS2IP(OKM) mod r
    // 4. return SK

    const uint8_t info[1] = {0};
    const size_t infoLen = 0;

    // Required by the ietf spec to be at least 32 bytes
    if (seed.size() < 32) {
        throw std::invalid_argument("Seed size must be at least 32 bytes");
    }

    // "BLS-SIG-KEYGEN-SALT-" in ascii
    const uint8_t saltHkdf[20] = {66, 76, 83, 45, 83, 73, 71, 45, 75, 69,
                                89, 71, 69, 78, 45, 83, 65, 76, 84, 45};

    uint8_t *prk = Util::SecAlloc<uint8_t>(32);
    uint8_t *ikmHkdf = Util::SecAlloc<uint8_t>(seed.size() + 1);
    memcpy(ikmHkdf, seed.begin(), seed.size());
    ikmHkdf[seed.size()] = 0;

    const uint8_t L = 48;  // `ceil((3 * ceil(log2(r))) / 16)`, where `r` is the
                        // order of the BLS 12-381 curve

    uint8_t *okmHkdf = Util::SecAlloc<uint8_t>(L);

    uint8_t keyInfoHkdf[infoLen + 2];
    memcpy(keyInfoHkdf, info, infoLen);
    keyInfoHkdf[infoLen] = 0;  // Two bytes for L, 0 and 48
    keyInfoHkdf[infoLen + 1] = L;

    HKDF256::ExtractExpand(
        okmHkdf,
        L,
        ikmHkdf,
        seed.size() + 1,
        saltHkdf,
        20,
        keyInfoHkdf,
        infoLen + 2);

    bn_t order;
    bn_new(order);
    g1_get_ord(order);

    // Make sure private key is less than the curve order
    bn_t *skBn = Util::SecAlloc<bn_t>(1);
    bn_new(*skBn);
    bn_read_bin(*skBn, okmHkdf, L);
    bn_mod_basic(*skBn, *skBn, order);

    uint8_t *skBytes = Util::SecAlloc<uint8_t>(32);
    bn_write_bin(skBytes, 32, *skBn);
    PrivateKey k = PrivateKey::FromBytes(Bytes(skBytes, 32));

    Util::SecFree(prk);
    Util::SecFree(ikmHkdf);
    Util::SecFree(skBn);
    Util::SecFree(okmHkdf);
    Util::SecFree(skBytes);

    return k;
}

void HDKeys::IKMToLamportSk(uint8_t* outputLamportSk, const uint8_t* ikm, size_t ikmLen, const uint8_t* salt, size_t saltLen)  {
    // Expands the ikm to 255*HASH_LEN bytes for the lamport sk
    const uint8_t info[1] = {0};
    HKDF256::ExtractExpand(outputLamportSk, HASH_LEN * 255, ikm, ikmLen, salt, saltLen, info, 0);
}

void HDKeys::ParentSkToLamportPK(uint8_t* outputLamportPk, const PrivateKey& parentSk, uint32_t index) {
    uint8_t* salt = Util::SecAlloc<uint8_t>(4);
    uint8_t* ikm = Util::SecAlloc<uint8_t>(HASH_LEN);
    uint8_t* notIkm = Util::SecAlloc<uint8_t>(HASH_LEN);
    uint8_t* lamport0 = Util::SecAlloc<uint8_t>(HASH_LEN * 255);
    uint8_t* lamport1 = Util::SecAlloc<uint8_t>(HASH_LEN * 255);

    Util::IntToFourBytes(salt, index);
    parentSk.Serialize(ikm);

    for (size_t i = 0; i < HASH_LEN; i++) {  // Flips the bits
        notIkm[i] = ikm[i] ^ 0xff;
    }

    HDKeys::IKMToLamportSk(lamport0, ikm, HASH_LEN, salt, 4);
    HDKeys::IKMToLamportSk(lamport1, notIkm, HASH_LEN, salt, 4);

    uint8_t* lamportPk = Util::SecAlloc<uint8_t>(HASH_LEN * 255 * 2);

    for (size_t i = 0; i < 255; i++) {
        Util::Hash256(lamportPk + i * HASH_LEN, lamport0 + i * HASH_LEN, HASH_LEN);
    }

    for (size_t i=0; i < 255; i++) {
        Util::Hash256(lamportPk + 255 * HASH_LEN + i * HASH_LEN, lamport1 + i * HASH_LEN, HASH_LEN);
    }
    Util::Hash256(outputLamportPk, lamportPk, HASH_LEN * 255 * 2);

    Util::SecFree(salt);
    Util::SecFree(ikm);
    Util::SecFree(notIkm);
    Util::SecFree(lamport0);
    Util::SecFree(lamport1);
    Util::SecFree(lamportPk);
}

PrivateKey HDKeys::DeriveChildSk(const PrivateKey& parentSk, uint32_t index) {
    uint8_t* lamportPk = Util::SecAlloc<uint8_t>(HASH_LEN);
    HDKeys::ParentSkToLamportPK(lamportPk, parentSk, index);
    std::vector<uint8_t> lamportPkVector(lamportPk, lamportPk + HASH_LEN);
    PrivateKey child = HDKeys::KeyGen(lamportPkVector);
    Util::SecFree(lamportPk);
    return child;
}

PrivateKey HDKeys::DeriveChildSkUnhardened(const PrivateKey& parentSk, uint32_t index) {
    uint8_t* buf = Util::SecAlloc<uint8_t>(G1Element::SIZE + 4);
    uint8_t* digest = Util::SecAlloc<uint8_t>(HASH_LEN);
    memcpy(buf, parentSk.GetG1Element().Serialize().data(), G1Element::SIZE);
    Util::IntToFourBytes(buf + G1Element::SIZE, index);
    Util::Hash256(digest, buf, G1Element::SIZE + 4);

    PrivateKey ret = PrivateKey::Aggregate({parentSk, PrivateKey::FromBytes(Bytes(digest, HASH_LEN), true)});

    Util::SecFree(buf);
    Util::SecFree(digest);
    return ret;
}

G1Element HDKeys::DeriveChildG1Unhardened(const G1Element& pk, uint32_t index) {
    uint8_t* buf = Util::SecAlloc<uint8_t>(G1Element::SIZE + 4);
    uint8_t* digest = Util::SecAlloc<uint8_t>(HASH_LEN);
    memcpy(buf, pk.Serialize().data(), G1Element::SIZE);

    Util::IntToFourBytes(buf + G1Element::SIZE, index);
    Util::Hash256(digest, buf, G1Element::SIZE + 4);

    bn_t nonce, ord;
    bn_new(nonce);
    bn_zero(nonce);
    bn_read_bin(nonce, digest, HASH_LEN);
    bn_new(ord);
    g1_get_ord(ord);
    bn_mod_basic(nonce, nonce, ord);

    Util::SecFree(buf);
    Util::SecFree(digest);

    G1Element gen = G1Element::Generator();
    return pk + gen * nonce;
}

G2Element HDKeys::DeriveChildG2Unhardened(const G2Element& pk, uint32_t index) {
    uint8_t* buf = Util::SecAlloc<uint8_t>(G2Element::SIZE + 4);
    uint8_t* digest = Util::SecAlloc<uint8_t>(HASH_LEN);
    memcpy(buf, pk.Serialize().data(), G2Element::SIZE);
    Util::IntToFourBytes(buf + G2Element::SIZE, index);
    Util::Hash256(digest, buf, G2Element::SIZE + 4);

    bn_t nonce, ord;
    bn_new(nonce);
    bn_zero(nonce);
    bn_read_bin(nonce, digest, HASH_LEN);
    bn_new(ord);
    g1_get_ord(ord);
    bn_mod_basic(nonce, nonce, ord);

    Util::SecFree(buf);
    Util::SecFree(digest);

    G2Element gen = G2Element::Generator();
    return pk + gen * nonce;
}
} // namespace bls
