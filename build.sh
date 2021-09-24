for SRC in ./*.rs
do
	rustc $SRC --out-dir target
done