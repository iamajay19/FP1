/* PQC-SECRET-003: link only into the isolated Rust regression executable.
 * All key/ciphertext/output bytes are synthetic public patterns. No primitive
 * operation is delegated to liboqs, and no production injection hook exists. */
#include <oqs/oqs.h>
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static uintptr_t watched_address;
static size_t watched_length;
static unsigned phase, writes, cleanses, frees;

static int selected(const char *operation) {
    const char *mode = getenv("CADP_TEST_PARTIAL_OUTPUT");
    assert(mode);
    return strcmp(mode, operation) == 0;
}

static void partial_output(uint8_t *output, size_t length) {
    assert(!watched_address && phase == 0);
    /* Rust allocated a zeroed fixed-size buffer. Simulate a partial native
     * write, keeping the untouched suffix zero. Never cleanse here. */
    for (size_t i = 0; i < length; ++i) assert(output[i] == 0);
    memset(output, 0xAB, length / 2);
    watched_address = (uintptr_t)output;
    watched_length = length;
    phase = 1;
    ++writes;
}

OQS_STATUS __wrap_OQS_KEM_keypair(const OQS_KEM *k, uint8_t *pk, uint8_t *sk) {
    assert(k && k->length_public_key == 1184 && k->length_secret_key == 2400);
    if (selected("kem-keygen")) {
        partial_output(sk, k->length_secret_key);
        return OQS_ERROR;
    }
    /* Only setup for encapsulation/decapsulation. This synthetic key is never
     * used by a real primitive: those operations are also intercepted below. */
    memset(pk, 0, k->length_public_key);
    memset(sk, 0x42, k->length_secret_key);
    return OQS_SUCCESS;
}

OQS_STATUS __wrap_OQS_SIG_keypair(const OQS_SIG *s, uint8_t *pk, uint8_t *sk) {
    (void)pk;
    assert(s && s->length_secret_key == 4032 && selected("sig-keygen"));
    partial_output(sk, s->length_secret_key);
    return OQS_ERROR;
}

OQS_STATUS __wrap_OQS_KEM_encaps(const OQS_KEM *k, uint8_t *ct, uint8_t *ss,
                                const uint8_t *pk) {
    (void)ct; (void)pk;
    assert(k && k->length_shared_secret == 32 && selected("encaps"));
    partial_output(ss, k->length_shared_secret);
    return OQS_ERROR;
}

OQS_STATUS __wrap_OQS_KEM_decaps(const OQS_KEM *k, uint8_t *ss, const uint8_t *ct,
                                const uint8_t *sk) {
    (void)ct; (void)sk;
    assert(k && k->length_shared_secret == 32 && selected("decaps"));
    partial_output(ss, k->length_shared_secret);
    return OQS_ERROR;
}

void __real_OQS_MEM_cleanse(void *pointer, size_t length);
void __wrap_OQS_MEM_cleanse(void *pointer, size_t length) {
    const int observed = watched_address && (uintptr_t)pointer == watched_address;
    if (observed) {
        assert(phase == 1 && length == watched_length);
        const uint8_t *bytes = pointer;
        for (size_t i = 0; i < length; ++i)
            assert(bytes[i] == (i < length / 2 ? 0xAB : 0));
    }
    /* Called by production SecretBuffer::Drop, after the FFI error. The
     * fixture neither initiates cleanup nor substitutes its implementation. */
    __real_OQS_MEM_cleanse(pointer, length);
    if (observed) {
        const uint8_t *bytes = pointer;
        for (size_t i = 0; i < length; ++i) assert(bytes[i] == 0);
        phase = 2;
        ++cleanses;
    }
}

void __real_free(void *pointer);
void __wrap_free(void *pointer) {
    if (watched_address && (uintptr_t)pointer == watched_address) {
        /* Inspect only before the actual free. Retain no dangling pointer and
         * never read memory after deallocation. Duplicate cleanup also fails. */
        assert(phase == 2);
        const uint8_t *bytes = pointer;
        for (size_t i = 0; i < watched_length; ++i) assert(bytes[i] == 0);
        watched_address = 0;
        phase = 3;
        ++frees;
    }
    __real_free(pointer);
}

/* Runs after the Rust driver returns. Missing Drop, cleansing or deallocation
 * fails even if the driver observed the expected typed operation error. */
__attribute__((destructor)) static void assert_complete(void) {
    assert(writes == 1 && cleanses == 1 && frees == 1);
    assert(phase == 3 && !watched_address);
    puts("PQC-SECRET-003 partial write -> error -> Rust Drop -> cleanse -> free PASS");
}
