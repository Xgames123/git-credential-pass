# script to verify versions


cargo_ver=$(rg -or "\$1" "^version[\t ]*=[\t ]*\"([0-9]\.[0-9]\.[0-9])\"" Cargo.toml)
echo "$cargo_ver"
pkg_ver=$(rg -or "\$1" "^pkgver=[\t ]*([0-9]\.[0-9]\.[0-9])" packaging/PKGBUILD)


if [ "$cargo_ver" != "$pkg_ver" ] ; then
  echo "Cargo.toml version: $cargo_ver doesn't match package version: $pkg_ver"
  exit -1
fi
