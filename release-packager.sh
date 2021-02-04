package="tide-webserver-0.3.2-x86_64-unknown-linux-gnu"
mkdir $package
cargo build --release
cp ./target/release/tide-webserver $package
cp ./web $package/web -R
cp ./keys $package/keys -R
cp release-run.sh $package/run.sh

cd $package
upx --best tide-webserver

tar -czvf $package.tar.gz *
mv $package.tar.gz ../$package.tar.gz

cd ..
rm $package -Rf

echo "Package built to $PWD/$package.tar.gz"