#/bin/sh
cc -o main main.c
./main '5+20-5' > tmp.s
cc -o tmp tmp.s
./tmp
echo $?
