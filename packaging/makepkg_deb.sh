# A crappy makepkg clone to build for debian
# Usage makepkg_deb
os="deb"

source ./PKGBUILD


srcdir=$PWD/src
pkgdir=$PWD/pkg/$pkgname

mkdir -p $srcdir
mkdir -p $pkgdir
mkdir -p $pkgdir/DEBIAN

echo "Downloading sources..."
for x_srcfile in "${source[@]}" ; do
  file_name=${x_srcfile%::*}
  download_url=${x_srcfile#*::}
  echo ${download_url//[$'\t\r\n ']}
  
  http_code=$(curl -L -s -o $file_name -w "%{http_code}" ${download_url//[$'\t\r\n ']})
  if [ "$http_code" == "404" ] ; then
    echo "Failed to download sources. status: $http_code"
    exit 1
  fi
  tar -xf $file_name -C $srcdir
done

debarch=$arch
if [ "$arch" = "x86_64" ]; then
  debarch="amd64"
fi
if [ "$arch" = "armv7h" ]; then
  debarch="armhf"
fi

x_debdepends=""
for x_deb in "${depends[@]}" ; do
  x_debdepends="$x_debdepends, $x_deb"
done

x_debconflicts=""
for x_deb in "${conflicts[@]}" ; do
  x_debconflicts="$x_debconflicts, $x_deb"
done

x_debprovides=""
for x_deb in "${provides[@]}" ; do
  x_debprovides="$x_debprovides, $x_deb"
done

x_controlfile=$pkgdir/DEBIAN/control
if [ -f $x_controlfile ] ; then
  rm -f $x_controlfile
fi
echo "Package: $pkgname" >> $x_controlfile
echo "Description: $pkgdesc" >> $x_controlfile
echo "Version: $pkgver" >> $x_controlfile
echo "Maintainer: $maintainer" >> $x_controlfile
echo "Architecture: $debarch" >> $x_controlfile

if [ "$x_debconflicts" != "" ] ; then
  echo "Depends: $x_debdepends"
fi
if [ "$x_debconflicts" != "" ] ; then
  echo "Conflicts: $x_debconflicts"
fi
if [ "$x_debprovides" != "" ] ; then
  echo "Provides: $x_debprovides"
fi

if [ "$url" != "" ] ; then
  echo "Homepage: $url" >> $x_controlfile
fi

if [ "$section" = "" ] ; then
  echo "FAIL: section is required for deb packages"
  exit 1
fi
echo "Section: $section" >> $x_controlfile

x_cwd=$PWD

echo "RUNNING prepare()"
prepare

echo "RUNNING build()"
build

echo "RUNNING package()"
package

cd $x_cwd
dpkg --build $pkgdir
mv $pkgdir/../$pkgname.deb $pkgname-$pkgver.deb
