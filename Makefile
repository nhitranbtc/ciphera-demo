SHELL=/bin/bash
all:
	@make help

# variant declaration

NODE_BIN=ciphera-collator

.PHONY: help ## Display help commands
help:
	@printf 'Usage:\n'
	@printf '  make <target>\n'
	@printf '\n'
	@printf 'Targets:\n'
	@IFS=$$'\n' ; \
    help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//'`); \
    for help_line in $${help_lines[@]}; do \
        IFS=$$'#' ; \
        help_split=($$help_line) ; \
        help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		IFS=$$':' ; \
		phony_command=($$help_split); \
        help_command=`echo $${phony_command[1]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		printf "  %-50s %s\n" $$help_command $$help_info ; \
    done


.PHONY: build-parachain ## Build release parachain node
build-parachain:
	mkdir -p parachain/binaries && \
	cd parachain/binaries && \
	if [ ! -d "polkadot-sdk" ]; then \
		git clone https://github.com/paritytech/polkadot-sdk.git; \
	fi; \
	cd polkadot-sdk && git checkout stable2407 && cargo build --release && \
	cd .. && mkdir -p polkadot-stable2407 && find polkadot-sdk/target/release/ -maxdepth 1 -type f -exec cp {} polkadot-stable2407/ \;



# build release

.PHONY: build-node ## Build release node
build-node:
	@cd parachain && cargo build --locked -p ciphera-collator --release

.PHONY: build-runtime-ciphera ## Build ciphera release runtime
build-runtime-ciphera:
	@cd parachain && cargo build --locked -p ciphera-parachain-runtime --release

# Copy Chainspec

.PHONY: cp_chainspec ## Copy Chainspec
cp_chainspec:
	@cd parachain/binaries/scripts/demo && ./copy_chainspec.sh

# Chainspec
.PHONY: chainspec ## Generate Chainspec
chainspec:
	@cd parachain/binaries/scripts/demo && ./generate_chainspec.sh

# launch a local network

.PHONY: launch-network-demo ## Launch a local standalone node without relaychain network
launch-network-demo:
	@cd parachain/zombienet/scripts-demo && ./launch-network-demo.sh



.PHONY: kill-network-demo ## Kill all processes started by launch-network-demo.sh
kill-network-demo:
	@chmod +x parachain/zombienet/scripts-demo/kill-network-demo.sh

	@cd parachain/zombienet/scripts-demo && ./kill-network-demo.sh

.PHONY: tail-logs ## Tail the log files generated by launch-network-demo.sh
tail-logs:
	@chmod +x parachain/zombienet/scripts-demo/tail-logs.sh
	@cd parachain/zombienet/scripts-demo && ./tail-logs.sh


# Compile smart contract

.PHONY: compile-smart-contract
compile-smart-contract:
	@cd parachain/contracts/demo_ciphera/scripts && ./build-all-pop.sh

# Check smart contract

.PHONY: test-smart-contract
test-smart-contract:
	@cd parachain/contracts/demo_ciphera/scripts && ./e2e_test.sh
