prepare:
	pip3 install --user --upgrade yaplon

plist:
	echo `tput setaf 4`"Warning! Please check json22plist in your PATH like '~/Library/Python/3.9/bin'. Be sure you're run 'make prepare'" `tput sgr0`
	json22plist -i glottis1.json -o glottis1_pl.plist
	json22plist -i trombone0.json -o trombone0_pl.plist

json:
	cargo run --example pink-trombone

schema:
	cargo run --example pink-trombone --features jsonse

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
	
clean:
	cargo clean
