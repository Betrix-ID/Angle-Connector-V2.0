TARGET = Angle_use  # Target Name file Compile
SRC = src/main.rs
CARGO = cargo

all: banner build

build:
	@echo ">> Compiling Rust program..."
	$(CARGO) build --release # Menyertakan --release untuk build yang lebih optimal
	@echo ""
	@echo ">> Build finished successfully!"

# Menjalankan program
run:
	@echo ">> Running the program..."
	./target/release/$(TARGET)

# Banner tampilan keren
banner:
	@echo
	@echo
	@echo "     ☆=====================================☆"
	@echo
	@echo "      ~ Description Compile Rust Program"
	@echo
	@echo "      - Author             :  @UnixeID"
	@echo "      - Version            :  2.0.2"  # Update versi sesuai dengan Cargo.toml
	@echo "      - Release            :  05 - Mei - 2025"  # Anda bisa memperbarui ini sesuai kebutuhan
	@echo "      - Project Name       :  Angle_use"  # Update nama proyek sesuai dengan Cargo.toml
	@echo
	@echo "    |_______________________________________|"
	@echo "     \\______________________________________/"
	@echo
	@echo
	@sleep 1

# Clean build files
clean:
	@echo ">> Cleaning build files..."
	@rm -f Cargo.lock  
	@find target/release -type f ! -name '$(TARGET)' -exec rm -f {} +
	@find target/debug -type f ! -name '$(TARGET)' -exec rm -f {} +
	@find target -type d -empty -exec rmdir {} +
	@echo ">> Cleaned build files."

# Format source code
format:
	@echo ">> Formatting Rust source code..."
	$(CARGO) fmt
	@echo ">> Source code formatted."