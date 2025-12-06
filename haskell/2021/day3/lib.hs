import Data.List (transpose)
import System.Environment (getArgs)
import Test.Hspec

createVal :: [[Char]] -> [Char]
createVal (h : t) = show (fromEnum (averageChar h >= 1)) ++ createVal t
createVal [] = []

averageChar :: [Char] -> Int
averageChar ('0' : t) = -1 + averageChar t
averageChar ('1' : t) = 1 + averageChar t
averageChar _h = 0

inverseDigits :: [Char] -> [Char]
inverseDigits (h : t) = show (fromEnum (read [h] == 0)) ++ inverseDigits t
inverseDigits [] = []

binaryToDecimal :: String -> Int
binaryToDecimal = foldl (\acc bit -> acc * 2 + read [bit]) 0

part1 :: String -> Int
part1 input =
  let binary = createVal (transpose (lines input))
  in (binaryToDecimal binary) * (binaryToDecimal (inverseDigits binary))

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 198
