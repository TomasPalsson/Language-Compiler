FILE      ?= examples/basic.bonk
BUILD_DIR  = build
ASM        = $(BUILD_DIR)/output.asm
OBJ        = $(BUILD_DIR)/output.o
RT_OBJ     = $(BUILD_DIR)/http.o
BIN        = $(BUILD_DIR)/prog

.PHONY: cargo-build compile assemble runtime link run clean

cargo-build:
	cargo build

compile: cargo-build
	@mkdir -p $(BUILD_DIR)
	cargo run -- $(FILE) $(ASM)

assemble: compile
	nasm -f macho64 $(ASM) -o $(OBJ)

runtime:
	@mkdir -p $(BUILD_DIR)
	gcc -arch x86_64 -c runtime/http.c -o $(RT_OBJ)

link: assemble runtime
	gcc -arch x86_64 $(OBJ) $(RT_OBJ) -lcurl -o $(BIN)

run: link
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR)
	cargo clean
