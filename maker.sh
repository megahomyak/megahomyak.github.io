export ROOT=$(pwd)
for d in $(find . -type d -not -path '*/.*')
do
    if [ -f "${d}/Makefile" ]; then
        make -f "Makefile" -s -C "${d}"
    else
        make -f "${ROOT}/Makerfile" -s -C "${d}"
    fi
done
