for VARIABLE in ./one.rs ./two.rs ./three.rs #./four.rs ./five.rs ./six.rs ./seven.rs ./eight.rs
do
	rustc $VARIABLE --out-dir target
done