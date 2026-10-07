#include "oqs_shim.h"
#include <oqs/oqs.h>
#include <string.h>

/* Retain pthread_once for the internal SHA3 dispatch pointers. The AVX-512
 * branch also mutates callback tables outside readers' synchronization (H-01),
 * so it must be excluded. This policy assumes no OpenSSL global objects. */
#if !defined(OQS_USE_PTHREADS) || defined(OQS_USE_OPENSSL)
#error "Phase 1 requires the approved pthread-enabled, OpenSSL-free configuration"
#endif
#if defined(OQS_USE_SHA3_AVX512VL)
#error "Phase 1 forbids OQS_USE_SHA3_AVX512VL"
#endif

/* The native structs stay in C. These checks guard every operation's buffer
 * contract even if a later Rust edit accidentally bypasses its validation. */
static int kem_metadata(const OQS_KEM *k) {
    return k && k->method_name && k->alg_version &&
           strcmp(k->method_name, OQS_KEM_alg_ml_kem_768) == 0 &&
           strcmp(k->alg_version, "FIPS203") == 0 && k->keypair && k->encaps && k->decaps &&
           k->length_public_key == 1184 && k->length_secret_key == 2400 &&
           k->length_ciphertext == 1088 && k->length_shared_secret == 32;
}

static int sig_metadata(const OQS_SIG *s) {
    return s && s->method_name && s->alg_version &&
           strcmp(s->method_name, OQS_SIG_alg_ml_dsa_65) == 0 &&
           strcmp(s->alg_version, "FIPS204") == 0 && s->keypair && s->sign && s->verify &&
           s->sign_with_ctx_str && s->verify_with_ctx_str && s->sig_with_ctx_support &&
           s->length_public_key == 1952 && s->length_secret_key == 4032 &&
           s->length_signature == 3309;
}

int cadp_initialize(size_t sizes[7]) {
    OQS_init();
    const char *version = OQS_version();
    if (!sizes || !version || strcmp(version, "0.16.0") != 0) return CADP_METADATA;
    if (!OQS_KEM_alg_is_enabled(OQS_KEM_alg_ml_kem_768) ||
        !OQS_SIG_alg_is_enabled(OQS_SIG_alg_ml_dsa_65)) return CADP_UNAVAILABLE;
    /* Minimal build must expose exactly one enabled algorithm in each family. */
    int kems = 0, sigs = 0;
    for (int i = 0; i < OQS_KEM_alg_count(); ++i) {
        const char *name = OQS_KEM_alg_identifier((size_t)i);
        if (!name) return CADP_METADATA;
        kems += OQS_KEM_alg_is_enabled(name) != 0;
    }
    for (int i = 0; i < OQS_SIG_alg_count(); ++i) {
        const char *name = OQS_SIG_alg_identifier((size_t)i);
        if (!name) return CADP_METADATA;
        sigs += OQS_SIG_alg_is_enabled(name) != 0;
    }
    if (kems != 1 || sigs != 1) return CADP_METADATA;
    OQS_KEM *k = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    OQS_SIG *s = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    int result = CADP_OK;
    if (!k || !s) result = CADP_ALLOCATION;
    else if (!kem_metadata(k) || !sig_metadata(s)) result = CADP_METADATA;
    else {
        sizes[0] = k->length_public_key;
        sizes[1] = k->length_secret_key;
        sizes[2] = k->length_ciphertext;
        sizes[3] = k->length_shared_secret;
        sizes[4] = s->length_public_key;
        sizes[5] = s->length_secret_key;
        sizes[6] = s->length_signature;
    }
    OQS_KEM_free(k);
    OQS_SIG_free(s);
    return result;
}

void cadp_thread_stop(void) { OQS_thread_stop(); }
void cadp_cleanse(uint8_t *buffer, size_t length) { OQS_MEM_cleanse(buffer, length); }
int cadp_secret_equal(const uint8_t *a, const uint8_t *b, size_t length) {
    return OQS_MEM_secure_bcmp(a, b, length) == 0;
}

int cadp_kem_keypair(uint8_t *pk, size_t pk_len, uint8_t *sk, size_t sk_len) {
    if (!pk || !sk || pk_len != 1184 || sk_len != 2400) return CADP_LENGTH;
    OQS_KEM *k = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!k) return CADP_ALLOCATION;
    int result = !kem_metadata(k) ? CADP_METADATA :
                 OQS_KEM_keypair(k, pk, sk) == OQS_SUCCESS ? CADP_OK : CADP_OPERATION;
    OQS_KEM_free(k);
    return result;
}

int cadp_kem_encaps(const uint8_t *pk, size_t pk_len, uint8_t *ct, size_t ct_len,
                    uint8_t *ss, size_t ss_len) {
    if (!pk || !ct || !ss || pk_len != 1184 || ct_len != 1088 || ss_len != 32)
        return CADP_LENGTH;
    OQS_KEM *k = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!k) return CADP_ALLOCATION;
    int result = !kem_metadata(k) ? CADP_METADATA :
                 OQS_KEM_encaps(k, ct, ss, pk) == OQS_SUCCESS ? CADP_OK : CADP_OPERATION;
    OQS_KEM_free(k);
    return result;
}

int cadp_kem_decaps(const uint8_t *sk, size_t sk_len, const uint8_t *ct, size_t ct_len,
                    uint8_t *ss, size_t ss_len) {
    if (!sk || !ct || !ss || sk_len != 2400 || ct_len != 1088 || ss_len != 32)
        return CADP_LENGTH;
    OQS_KEM *k = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!k) return CADP_ALLOCATION;
    int result = !kem_metadata(k) ? CADP_METADATA :
                 OQS_KEM_decaps(k, ss, ct, sk) == OQS_SUCCESS ? CADP_OK : CADP_OPERATION;
    OQS_KEM_free(k);
    return result;
}

int cadp_sig_keypair(uint8_t *pk, size_t pk_len, uint8_t *sk, size_t sk_len) {
    if (!pk || !sk || pk_len != 1952 || sk_len != 4032) return CADP_LENGTH;
    OQS_SIG *s = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    if (!s) return CADP_ALLOCATION;
    int result = !sig_metadata(s) ? CADP_METADATA :
                 OQS_SIG_keypair(s, pk, sk) == OQS_SUCCESS ? CADP_OK : CADP_OPERATION;
    OQS_SIG_free(s);
    return result;
}

int cadp_sig_sign(const uint8_t *sk, size_t sk_len, const uint8_t *message,
                  size_t message_len, uint8_t *signature, size_t capacity,
                  size_t *written, int empty_context_api) {
    if (!sk || !message || !signature || !written || sk_len != 4032 || capacity != 3309)
        return CADP_LENGTH;
    if (empty_context_api != 0 && empty_context_api != 1) return CADP_METADATA;
    *written = 0;
    OQS_SIG *s = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    if (!s) return CADP_ALLOCATION;
    int result = CADP_METADATA;
    if (sig_metadata(s)) {
        OQS_STATUS status = empty_context_api ?
            OQS_SIG_sign_with_ctx_str(s, signature, written, message, message_len, NULL, 0, sk) :
            OQS_SIG_sign(s, signature, written, message, message_len, sk);
        result = status == OQS_SUCCESS ? CADP_OK : CADP_OPERATION;
    }
    OQS_SIG_free(s);
    return result;
}

int cadp_sig_verify(const uint8_t *pk, size_t pk_len, const uint8_t *message,
                    size_t message_len, const uint8_t *signature, size_t length,
                    int empty_context_api) {
    if (!pk || !message || !signature || pk_len != 1952 || length != 3309)
        return CADP_LENGTH;
    if (empty_context_api != 0 && empty_context_api != 1) return CADP_METADATA;
    OQS_SIG *s = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    if (!s) return CADP_ALLOCATION;
    int result = CADP_METADATA;
    if (sig_metadata(s)) {
        OQS_STATUS status = empty_context_api ?
            OQS_SIG_verify_with_ctx_str(s, message, message_len, signature, length, NULL, 0, pk) :
            OQS_SIG_verify(s, message, message_len, signature, length, pk);
        result = status == OQS_SUCCESS ? CADP_OK : CADP_INVALID_SIGNATURE;
    }
    OQS_SIG_free(s);
    return result;
}
