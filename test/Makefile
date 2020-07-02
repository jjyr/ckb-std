CONTRACTS := "contract" "overflow-contract"

test: clean build patch
	cargo test -- --nocapture

build:
	for C in ${CONTRACTS}; do \
		cd $$C && cargo build --release && cd -; \
	done

C := contract/target/riscv64imac-unknown-none-elf/release/contract
patch:
	for C in ${CONTRACTS}; do \
		ckb-binary-patcher -i $$C/target/riscv64imac-unknown-none-elf/release/$$C -o $$C/target/riscv64imac-unknown-none-elf/release/$$C; \
	done

clean:
	for C in ${CONTRACTS}; do \
		echo $$C; \
		cd $$C && cargo clean && cd -; \
	done
