#ifndef CADP_OQS_SHIM_H
#define CADP_OQS_SHIM_H

#include <stddef.h>
#include <stdint.h>

/* Private shim status codes; none contain key material. */
enum {
    CADP_OK = 0,
    CADP_UNAVAILABLE = 1,
    CADP_METADATA = 2,
    CADP_ALLOCATION = 3,
    CADP_OPERATION = 4,
    CADP_INVALID_SIGNATURE = 5,
    CADP_LENGTH = 6
};

/* Called once by Rust OnceLock; numeric metadata uses C size_t only. */
int cadp_initialize(size_t sizes[7]);
void cadp_thread_stop(void);
void cadp_cleanse(uint8_t *buffer, size_t length);
int cadp_secret_equal(const uint8_t *a, const uint8_t *b, size_t length);

int cadp_kem_keypair(uint8_t *pk, size_t pk_len, uint8_t *sk, size_t sk_len);
int cadp_kem_encaps(const uint8_t *pk, size_t pk_len, uint8_t *ct, size_t ct_len,
                    uint8_t *ss, size_t ss_len);
int cadp_kem_decaps(const uint8_t *sk, size_t sk_len, const uint8_t *ct, size_t ct_len,
                    uint8_t *ss, size_t ss_len);
int cadp_sig_keypair(uint8_t *pk, size_t pk_len, uint8_t *sk, size_t sk_len);
/* empty_context_api selects ordinary (0) or NULL,0 context-aware (1) APIs only. */
int cadp_sig_sign(const uint8_t *sk, size_t sk_len, const uint8_t *message,
                  size_t message_len, uint8_t *signature, size_t capacity,
                  size_t *written, int empty_context_api);
int cadp_sig_verify(const uint8_t *pk, size_t pk_len, const uint8_t *message,
                    size_t message_len, const uint8_t *signature, size_t length,
                    int empty_context_api);

#endif
