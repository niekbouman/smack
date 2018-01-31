#! /bin/bash

echo
echo "-----------------------------------------------------------------------"
echo "Running functions"
./regtest.py --folder functions

echo
echo "-----------------------------------------------------------------------"
echo "Running recursion"
./regtest.py --folder recursion

echo
echo "-----------------------------------------------------------------------"
echo "Running generics"
./regtest.py --folder generics

echo
echo "-----------------------------------------------------------------------"
echo "Running ifc"
./regtest.py --folder ifc

echo
echo "-----------------------------------------------------------------------"
echo "Running ifc_bug"
./regtest.py --folder ifc_bug

echo
echo "-----------------------------------------------------------------------"
echo "Running ifc_bug"
./regtest.py --folder ifc_bug

echo
echo "-----------------------------------------------------------------------"
echo "Running loops"
./regtest.py --folder loops

echo
echo "-----------------------------------------------------------------------"
echo "Running ops"
./regtest.py --folder ops

echo
echo "-----------------------------------------------------------------------"
echo "Running structures"
./regtest.py --folder structures

echo
echo "-----------------------------------------------------------------------"
echo "Running vector"
./regtest.py --folder vector

cd cross-language/incr
echo
echo "-----------------------------------------------------------------------"
echo "Running cross-language"
echo
echo "-----------------------------------------------------------------------"
echo "Running incr_test"
echo "Result:"
./test.sh |& tail -1
echo "Expected: SMACK found no errors with unroll bound 1."

echo
echo "-----------------------------------------------------------------------"
echo "Running incr_test_fail"
echo "Result:"
./test_fail.sh |& tail -1
echo "Expected: SMACK found an error."

cd ../vector_fibonacci
echo
echo "-----------------------------------------------------------------------"
echo "Running vector_fibonacci"
echo "Result:"
./test.sh |& tail -1
echo "Expected: SMACK found no errors with unroll bound 5."

echo
echo "-----------------------------------------------------------------------"
echo "Running vector_fibonacci_fail"
echo "Result:"
./test_fail.sh |& tail -1
echo "Expected: SMACK found an error."

cd ../../

