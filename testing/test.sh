cargo build
export RUST_BACKTRACE=full
bin_path="../target/debug/git-credential-pass"

payload=$'username=ldev\nhost=git.ldev.eu.org\npassword=passwd\n'


run_test()
{
  echo "TEST $0"
  echo "$payload" | $bin_path -vvv -t $1 -p $2 $3

  if [ $? != $4 ] ; then
    echo "TEST FAILED expected exitcode: $4"
    exit -1
  fi
}

