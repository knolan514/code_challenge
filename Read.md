Challenge 1:
Instructions from the Wizards

	The door will display 3 numbers described below. You will need to read three numbers in as command line arguments:

	-first divisor - This is the first divisor you will check against the dividend.
	-second divisor - This is the second divisor you will check against the dividend.
	-upper bound - This is the maximum dividend.

	You script will cycle through the range of [0..<upper bound>] and print one of the following strings:

	-If divisible by both the first divisor and second divisor print FizzBuzz
	-If divisible by just the first divisor then print Fizz
	-If divisible by just the second divisor then print Buzz
	-If not divisible by either divisor then print the dividend itself.

Instructions:
	In Terminal:
		$ ghc -o fizz_buzz fizz_buzz.hs (I also found out $ ghc --make fizz_buzz.hs works too)
		$ ./fizz_buzz 2 4 6 (or any 3 numbers)
