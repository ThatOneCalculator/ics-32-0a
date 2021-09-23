for SRC in ./*.rs
do
	rustc $SRC --out-dir target
done
for BIN in ./target/*
do
	$BIN
done