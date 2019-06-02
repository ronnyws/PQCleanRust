# This Makefile can be used with GNU Make or BSD Make

# override as desired
TYPE=kem
SCHEME=kyber768
IMPLEMENTATION=clean

SCHEME_DIR="../crypto_$(TYPE)/$(SCHEME)/$(IMPLEMENTATION)"
SCHEME_UPPERCASE=$(shell echo $(SCHEME) | tr a-z A-Z | sed 's/-//g')
IMPLEMENTATION_UPPERCASE=$(shell echo $(IMPLEMENTATION) | tr a-z A-Z | sed 's/-//g')

COMMON_DIR=../PQClean/common
TEST_DIR=../PQClean/test
COMMON_DIR_C2RUST=c/common

LIB_RUST_COMMON=../target/debug/libcommon.a
LIB_RUST_C2RUST=../target/debug/libc2rust.a

COMMON_FILES=$(COMMON_DIR_C2RUST)/aes.c \
    $(COMMON_DIR)/sha2.c $(COMMON_DIR)/fips202.c
COMMON_HEADERS=$(COMMON_DIR_C2RUST)/*.h $(COMMON_DIR)/*.h
DEST_DIR=../bin


# This -Wall was supported by the European Commission through the ERC Starting Grant 805031 (EPOQUE)
CFLAGS=-O3 -Wall -Wextra -Wpedantic -Wvla -Werror -std=c99 \
	   -Wundef -Wshadow -Wcast-align -Wpointer-arith -Wmissing-prototypes\
	   -fstrict-aliasing -fno-common -pipe \
	   -I$(COMMON_DIR) -I$(COMMON_DIR_C2RUST) $(EXTRAFLAGS)


all: $(DEST_DIR)/test_common_aes

.PHONY: $(LIB_RUST_COMMON)
$(LIB_RUST_COMMON):
	   cargo build --manifest-path ../common/Cargo.toml

.PHONY: $(LIB_RUST_C2RUST)
$(LIB_RUST_C2RUST):
	   cargo build --manifest-path ../c2rust/Cargo.toml

$(DEST_DIR)/test_common_aes: $(TEST_DIR)/common/aes.c $(LIB_RUST_COMMON) $(LIB_RUST_C2RUST) $(COMMON_FILES)
	mkdir -p $(DEST_DIR)
	$(CC) $(CFLAGS) $< $(COMMON_FILES) ../target/debug/libcommon.a ../target/debug/libc2rust.a -o $@

.PHONY: clean
clean:
	$(RM) $(DEST_DIR)/test_common_aes

.PHONY: distclean
distclean:
	$(RM) -r $(DEST_DIR)
