import secrets
import sys
import time

from dashbls import (
    AugSchemeMPL,
    G1Element,
    G2Element,
    PrivateKey,
)


def startStopwatch() -> float:
    return time.perf_counter()


def endStopwatch(test_name: str, start: float, numIters: int) -> None:
    end_time = time.perf_counter()

    duration = end_time - start

    print(
        f"\n{test_name}\nTotal: {numIters} runs in {duration * 1000:.1f} ms\n"
        f"Avg: {duration * 1000 / numIters:f}"
    )


def batch_verification() -> None:

    numIters = 100000
    sig_bytes = []
    pk_bytes = []
    ms = []

    for i in range(numIters):
        message = b"%d" % i
        sk: PrivateKey = AugSchemeMPL.key_gen(secrets.token_bytes(32))
        pk: G1Element = sk.get_g1()
        sig: G2Element = AugSchemeMPL.sign(sk, message)

        sig_bytes.append(bytes(sig))
        pk_bytes.append(bytes(pk))
        ms.append(message)

    pks = []

    start = startStopwatch()
    for pk in pk_bytes:
        pks.append(G1Element.from_bytes(pk))

    endStopwatch("Public key validation", start, numIters)

    sigs = []

    start = startStopwatch()
    for sig in sig_bytes:
        sigs.append(G2Element.from_bytes(sig))
    endStopwatch("Signature validation", start, numIters)

    start = startStopwatch()
    aggSig = AugSchemeMPL.aggregate(sigs)
    endStopwatch("Aggregation", start, numIters)

    start = startStopwatch()
    ok = AugSchemeMPL.aggregate_verify(pks, ms, aggSig)
    endStopwatch("Batch verification", start, numIters)
    if not ok:
        print("aggregate_verification failed!")
        sys.exit(1)


batch_verification()
