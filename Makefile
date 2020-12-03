SOURCEDIR=coverage

all: used_crate

%: $(SOURCEDIR)/lib/%.rs
	# Compile the test library with coverage instrumentation
	$(RUSTC) $(SOURCEDIR)/lib/$@.rs \
			$$( grep -q '^\/\/ require-rust-edition-2018' $(SOURCEDIR)/lib/$@.rs && \
				echo "--edition=2018" \
			) \
			-Zinstrument-coverage \
			--crate-type rlib
