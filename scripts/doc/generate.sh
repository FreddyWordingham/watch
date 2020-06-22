cd $WATCH_DIR;

cargo doc;
rm -r $WATCH_DIR/docs;
mv $WATCH_DIR/target/doc $WATCH_DIR/docs;
echo "<meta http-equiv=refresh content=0;url=watch/index.html>" > $WATCH_DIR/docs/index.html;

cd -;
