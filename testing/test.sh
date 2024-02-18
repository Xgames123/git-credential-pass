#!/bin/bash

if [ -d testing ] ; then
  cd testing
fi


echo "CREATING TEST FILES"
mkdir ~/gcp_testdir
mkdir ~/gcp_testdir/testdir2
cargo build
export RUST_BACKTRACE=full
bin_path="../target/debug/git-credential-pass"

payload=$'username=ldev\nhost=git.ldev.eu.org\npassword=passwd\n'

echo "DELETING dev password dir"
pass rm dev

echo "READY FOR TEST"
echo ""

run_test()
{
  echo ""
  echo "RUNNING $1"
  output=$(echo "$payload" | $bin_path -r 3 -vvv -t $2 -p $3 $4)

  if [ $? != $5 ] ; then
    echo "TEST FAILED: exit code: $? expected exitcode: $5"
    exit 1
  fi

  if [ "$output" != "$6" ] ; then
    echo "TEST FAILED: expected output:"
    echo "$6"
    echo "Got output:"
    echo "$output"
    exit 1
  fi
  echo ""
}

test_store_ok(){
  run_test "$1" "$2" "$3" store 0 "$4"
}
test_get_ok(){
  run_test $1 $2 $3 get 0 "$4"
}
test_erase_ok(){
  run_test $1 $2 $3 erase 0 "$4"
}

test_store_err(){
  run_test "$1" "$2" "$3" store 1 "$4"
}

pass_exist(){
  pass show $1
  if [ $? != 0 ] ; then
    echo "TEST FAILED: password not in store"
    exit 1
  fi
}
pass_not_exist(){
  pass show $1
  if [ $? == 0 ] ; then
    echo "TEST FAILED: password not deleted"
    exit 1
  fi
}

tmpls="templates"

test_store_ok "simple_store" "$tmpls/default.template" "dev/{host}" ""
pass_exist "dev/git.ldev.eu.org"
test_get_ok "simple_get" "$tmpls/default.template" "dev/{host}" $'password=passwd\nusername=ldev'
test_erase_ok "simple_erase" "$tmpls/default.template" "dev/{host}" ""
pass_not_exist "dev/git.ldev.eu.org"

test_store_err "invalid_token_order" "$tmpls/invalidtokenorder.template" "dev/invalid_token_order" ""

echo "ALL TESTS PASSED"
echo "running cargo test"
cargo test
exit 0
