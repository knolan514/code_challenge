import System.Environment
import Control.Monad
-- fizz_buzz function takes and integer and prints a string
-- fizz_buzz :: Int -> String
main = do
    args <- getArgs
    -- if the length of the arguments from the command line does not equal 3, and error will appear.
    if length(args) /= 3  then print "ERROR: Enter 3 numbers."
    else do
      -- converts the command line arguments from strings into integers.
      let numbers = map (read::String->Int) args
      -- if the number entered are less than 0, an error will appear.
      if any (<0) numbers then print "ERROR: No negative numbers."
        else do
          let firstDivisor = numbers!!0
          let secondDivisor = numbers!!1
          let upperBound = numbers!!2
          -- evaluate each number in sequence and use dividend as an anonymous parameter
          forM_ [1..upperBound] $ \dividend -> do
            -- if the guess is a modulus of the firstDivisor and secondDivisor it prints "FizzBuzz"
            if dividend `mod` firstDivisor == 0 && dividend `mod` secondDivisor == 0 then putStrLn "FizzBuzz"
            -- if the guess is a modulus firstDivisor it prints "Fizz"
            else if dividend `mod` firstDivisor == 0 then putStrLn "Fizz"
            -- if the guess is a modulus secondDivisor it prints "Buzz"
            else if dividend `mod` secondDivisor == 0 then putStrLn "Buzz"
            -- if it's none of these, it will just print the number
            else print dividend
