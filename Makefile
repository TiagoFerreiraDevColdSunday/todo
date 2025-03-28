install_todo:
	$(MAKE) install_cargo
	chmod +x install.sh
	./install.sh

install_cargo:
	curl https://sh.rustup.rs -sSf | sh

format:
	cargo fmt

build:
	$(MAKE) format
	./install.sh
