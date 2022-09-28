prepare:
	pip3 install --user --upgrade yaplon

plist:
	echo `tput setaf 4`"Warning! Please check json22plist in your PATH like '~/Library/Python/3.9/bin'. Be sure you're run 'make prepare'" `tput sgr0`
	json22plist -i glottis1.json -o glottis1_pl.plist
	json22plist -i trombone0.json -o trombone0_pl.plist
	json22plist -i trombone0_1.json -o trombone0_1_pl.plist
	json22plist -i trombone0_2.json -o trombone0_2_pl.plist
	json22plist -i trombone0_3.json -o trombone0_3_pl.plist
	json22plist -i trombone0_4.json -o trombone0_4_pl.plist
	json22plist -i trombone0_5.json -o trombone0_5_pl.plist
	json22plist -i trombone0_6.json -o trombone0_6_pl.plist
	json22plist -i trombone0_7.json -o trombone0_7_pl.plist
	json22plist -i trombone0_big.json -o trombone0_big_pl.plist

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
