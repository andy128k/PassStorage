#!/bin/sh

TMPDIR=/tmp/win32-build

rm -rf "${TMPDIR}"
mkdir -p "${TMPDIR}"
pushd "${TMPDIR}" > /dev/null
    mkdir -p var/lib/pacman
    mkdir -p var/log
    mkdir -p tmp

    pacman --noconfirm --needed -Syu --root "${TMPDIR}"
    pacman -S filesystem bash pacman --needed --noconfirm --root "${TMPDIR}"
    pacman --noconfirm --needed --root "${TMPDIR}" mingw-w64-x86_64-gtk3
popd > /dev/null
ls -la "${TMPDIR}"

7z a archive.zip "${TMPDIR}\"
