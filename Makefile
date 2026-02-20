FILE      ?= examples/basic.bonk
BUILD_DIR  = build
ASM        = $(BUILD_DIR)/output.asm
OBJ        = $(BUILD_DIR)/output.o
BIN        = $(BUILD_DIR)/prog

.PHONY: cargo-build compile assemble link run clean

cargo-build:
	cargo build

compile: cargo-build
	@mkdir -p $(BUILD_DIR)
	cargo run -- $(FILE) $(ASM)

assemble: compile
	nasm -f macho64 $(ASM) -o $(OBJ)

link: assemble
	gcc -arch x86_64 $(OBJ) -o $(BIN)

run: link
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR)
	cargo clean
