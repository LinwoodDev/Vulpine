#!/bin/bash

# Default values for architecture
DIRECTORY_ARCH="x64"  # Default directory name
BINARY_ARCH="x86_64"  # Default binary name
RPM_ARCH="x86_64"     # Default RPM architecture

# Parse command-line arguments
while getopts "d:b:" opt; do
  case $opt in
    d) DIRECTORY_ARCH="$OPTARG" ;;  # Set the directory architecture
    b) BINARY_ARCH="$OPTARG" ;;    # Set the binary architecture
    *) 
      echo "Usage: $0 [-d directory_arch] [-b binary_arch]"
      exit 1
      ;;
  esac
done
RPM_ARCH=$BINARY_ARCH

# Normalize architecture names for RPM
if [ "$RPM_ARCH" == "arm64" ]; then
  RPM_ARCH="aarch64"
fi
# Read version from pubspec
VULPINE_VERSION_REGEX="version:\s(.+)\+(.+)"
[[ $(grep -E "${VULPINE_VERSION_REGEX}" pubspec.yaml) =~ ${VULPINE_VERSION_REGEX} ]]
VULPINE_VERSION="${BASH_REMATCH[1]}"

# Replace - with ~ to match RPM versioning
RPM_VERSION=$(echo $VULPINE_VERSION | sed 's/-/~/g')
CURRENT_DIR=$(pwd)
echo "Building Vulpine $RPM_VERSION for $DIRECTORY_ARCH/$BINARY_ARCH ($RPM_ARCH)"

# Clean and set up build directories
rm -rf build/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
mkdir -p build/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Copy files
cp linux/rpm/linwood-vulpine.spec build/SPECS/linwood-vulpine.spec
cp -r build/linux/${DIRECTORY_ARCH}/release/bundle build/SOURCES/linwood-vulpine-$RPM_VERSION
chmod 755 build/SOURCES/linwood-vulpine-$RPM_VERSION/vulpine
mv build/SOURCES/linwood-vulpine-$RPM_VERSION/vulpine build/SOURCES/linwood-vulpine-$RPM_VERSION/linwood-vulpine
cp linux/rpm/linwood-vulpine.desktop build/SOURCES/linwood-vulpine-$RPM_VERSION/linwood-vulpine.desktop

# Update .spec file with the correct version
sed -i "2s/.*/Version: $RPM_VERSION/" build/SPECS/linwood-vulpine.spec

# Create tarball
cd build/SOURCES/
# Fix .so files using patchelf
cd linwood-vulpine-$RPM_VERSION/lib
for file in *.so; do
  PATCHELF_OUTPUT=$(patchelf --print-rpath "$file")
  echo "Checking $file: $PATCHELF_OUTPUT"
  # Skip file if PATCHELF_OUTPUT does not contain CURRENT_DIR
  if [[ ! $PATCHELF_OUTPUT =~ $CURRENT_DIR ]]; then
    echo "Skipping $file"
    continue
  fi
  echo "Fixing $file"
  patchelf --set-rpath '$ORIGIN' "$file"
done
cd ../../
tar --create --file linwood-vulpine-$RPM_VERSION.tar.gz linwood-vulpine-$RPM_VERSION
cd ../../

# Build RPM
QA_RPATHS=$[ 0x0001|0x0010 ] rpmbuild -bb build/SPECS/linwood-vulpine.spec --define "_topdir $(pwd)/build"

# Copy the RPM to the build folder
cp build/RPMS/${RPM_ARCH}/linwood-vulpine-*.rpm build/linwood-vulpine-linux-${BINARY_ARCH}.rpm
