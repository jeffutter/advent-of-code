tomorrow := $(shell date -d "tomorrow" +day%d)

$(tomorrow):
	cargo generate --path ../../rust-template --name $(tomorrow)
	sed -i -e 's/# \("*$(tomorrow)"*\)/\1/' Cargo.toml
	sed -i -e 's/\/\/ \($(tomorrow)\)/\1/' src/main.rs
	cd $(tomorrow) && cargo build

next: $(tomorrow)
