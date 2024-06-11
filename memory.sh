cargo build --release

for FILE in "./files/cal.com.tsx" "./files/typescript.js"
do
  echo $FILE

  for APP in oxc swc biome
  do
    hyperfine --warmup 10 --show-output "/usr/bin/time -al ./target/release/$APP $FILE > /dev/null" 2>&1 | \
      grep "maximum resident set size" | \
      awk '{ print $1 }' \
      > ./target/output

    TOTAL=$(awk '{ total += $1 } END { print total }' ./target/output)
    COUNT=$(wc -l ./target/output| awk '{ print $1 }')
    AVERAGE_MB=$(echo "$TOTAL $COUNT" | awk '{printf "%.1f", $1 / $2 / 1000 / 1000}')

    echo $APP $AVERAGE_MB mb
  done
done
