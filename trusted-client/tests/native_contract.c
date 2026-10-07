/* PQC-FFI-001..004: private shim guards/failure cleanup under ASan/UBSan.
 * Linker wrapping is confined to this disposable test binary, never production.
 * Artificial failure outputs are public byte patterns, not project secrets. */
#include "oqs_shim.h"
#include <oqs/oqs.h>
#include <assert.h>
#include <stdio.h>
#include <string.h>

static int fail_allocation, fail_operation, bad_metadata, null_version;
static int kem_live, sig_live, allocations;

OQS_KEM *__real_OQS_KEM_new(const char *name);
OQS_SIG *__real_OQS_SIG_new(const char *name);
void __real_OQS_KEM_free(OQS_KEM *key);
void __real_OQS_SIG_free(OQS_SIG *key);
const char *__real_OQS_version(void);

OQS_KEM *__wrap_OQS_KEM_new(const char *name) {
    ++allocations;
    if (fail_allocation) return NULL;
    OQS_KEM *key = __real_OQS_KEM_new(name);
    if (key) {
        ++kem_live;
        if (bad_metadata) key->method_name = NULL;
    }
    return key;
}
OQS_SIG *__wrap_OQS_SIG_new(const char *name) {
    ++allocations;
    if (fail_allocation) return NULL;
    OQS_SIG *key = __real_OQS_SIG_new(name);
    if (key) {
        ++sig_live;
        if (bad_metadata) key->alg_version = NULL;
    }
    return key;
}
void __wrap_OQS_KEM_free(OQS_KEM *key) {
    if (key) --kem_live;
    __real_OQS_KEM_free(key);
}
void __wrap_OQS_SIG_free(OQS_SIG *key) {
    if (key) --sig_live;
    __real_OQS_SIG_free(key);
}
const char *__wrap_OQS_version(void) {
    return null_version ? NULL : __real_OQS_version();
}

/* All these entry points are called with fail_operation set. A missed guard
 * fails an assertion instead of using deliberately invalid test keys. */
OQS_STATUS __wrap_OQS_KEM_keypair(const OQS_KEM *k, uint8_t *pk, uint8_t *sk) {
    (void)k; (void)pk;
    assert(fail_operation);
    memset(sk, 0xAB, 2400);
    return OQS_ERROR;
}
OQS_STATUS __wrap_OQS_KEM_encaps(const OQS_KEM *k, uint8_t *ct, uint8_t *ss,
                                const uint8_t *pk) {
    (void)k; (void)ct; (void)ss; (void)pk;
    assert(fail_operation);
    return OQS_ERROR;
}
OQS_STATUS __wrap_OQS_KEM_decaps(const OQS_KEM *k, uint8_t *ss, const uint8_t *ct,
                                const uint8_t *sk) {
    (void)k; (void)ss; (void)ct; (void)sk;
    assert(fail_operation);
    return OQS_ERROR;
}
OQS_STATUS __wrap_OQS_SIG_keypair(const OQS_SIG *s, uint8_t *pk, uint8_t *sk) {
    (void)s; (void)pk;
    assert(fail_operation);
    memset(sk, 0xAB, 4032);
    return OQS_ERROR;
}
OQS_STATUS __wrap_OQS_SIG_sign(const OQS_SIG *s, uint8_t *out, size_t *written,
                              const uint8_t *message, size_t length, const uint8_t *sk) {
    (void)s; (void)out; (void)written; (void)message; (void)length; (void)sk;
    assert(fail_operation);
    return OQS_ERROR;
}
OQS_STATUS __wrap_OQS_SIG_verify(const OQS_SIG *s, const uint8_t *message, size_t length,
                                const uint8_t *signature, size_t siglen, const uint8_t *pk) {
    (void)s; (void)message; (void)length; (void)signature; (void)siglen; (void)pk;
    assert(fail_operation);
    return OQS_ERROR;
}

static uint8_t kpk[1184], ksk[2400], ct[1088], ss[32];
static uint8_t spk[1952], ssk[4032], signature[3309];
static const uint8_t message[] = "public test";

static void six_operations(int expected, int verify_expected) {
    size_t written = 0;
    assert(cadp_kem_keypair(kpk, sizeof(kpk), ksk, sizeof(ksk)) == expected);
    assert(cadp_kem_encaps(kpk, sizeof(kpk), ct, sizeof(ct), ss, sizeof(ss)) == expected);
    assert(cadp_kem_decaps(ksk, sizeof(ksk), ct, sizeof(ct), ss, sizeof(ss)) == expected);
    assert(cadp_sig_keypair(spk, sizeof(spk), ssk, sizeof(ssk)) == expected);
    assert(cadp_sig_sign(ssk, sizeof(ssk), message, sizeof(message), signature,
                         sizeof(signature), &written, 0) == expected);
    assert(cadp_sig_verify(spk, sizeof(spk), message, sizeof(message), signature,
                           sizeof(signature), 0) == verify_expected);
    assert(kem_live == 0 && sig_live == 0);
}

int main(void) {
    size_t sizes[7] = {0};
    assert(cadp_initialize(sizes) == CADP_OK);
    assert(kem_live == 0 && sig_live == 0);
    int before = allocations;
    size_t written = 0;
    assert(cadp_kem_keypair(NULL, 1184, ksk, 2400) == CADP_LENGTH);
    assert(cadp_kem_encaps(kpk, 1183, ct, 1088, ss, 32) == CADP_LENGTH);
    assert(cadp_kem_decaps(ksk, 2400, ct, 1089, ss, 32) == CADP_LENGTH);
    assert(cadp_sig_keypair(spk, 1952, NULL, 4032) == CADP_LENGTH);
    assert(cadp_sig_sign(ssk, 4032, message, sizeof(message), signature, 3308,
                         &written, 0) == CADP_LENGTH);
    assert(cadp_sig_verify(spk, 1952, message, sizeof(message), signature, 0, 0)
           == CADP_LENGTH);
    assert(allocations == before);
    puts("PQC-FFI-001 native null/length guards PASS");

    fail_allocation = 1;
    six_operations(CADP_ALLOCATION, CADP_ALLOCATION);
    assert(cadp_initialize(sizes) == CADP_ALLOCATION);
    fail_allocation = 0;
    puts("PQC-FFI-002 allocation failure/cleanup PASS");

    fail_operation = 1;
    six_operations(CADP_OPERATION, CADP_INVALID_SIGNATURE);
    cadp_cleanse(ksk, sizeof(ksk));
    cadp_cleanse(ssk, sizeof(ssk));
    for (size_t i = 0; i < sizeof(ksk); ++i) assert(ksk[i] == 0);
    for (size_t i = 0; i < sizeof(ssk); ++i) assert(ssk[i] == 0);
    fail_operation = 0;
    puts("PQC-FFI-003 native operation failure/cleanup PASS");

    bad_metadata = 1;
    six_operations(CADP_METADATA, CADP_METADATA);
    assert(cadp_initialize(sizes) == CADP_METADATA);
    bad_metadata = 0;
    null_version = 1;
    assert(cadp_initialize(sizes) == CADP_METADATA);
    null_version = 0;
    assert(cadp_initialize(NULL) == CADP_METADATA);
    assert(kem_live == 0 && sig_live == 0);
    puts("PQC-FFI-004 inconsistent/null metadata rejection PASS");
    cadp_thread_stop();
    return 0;
}
