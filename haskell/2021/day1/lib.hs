import System.Environment (getArgs)
import Test.Hspec

readInputFile :: String -> [Int]
readInputFile content = map read (lines content)

average :: [Int] -> [Int] -> [Int]
average [_h] sumList = sumList
average (h : t) sumList = average t (sumList ++ [sum (take 3 (h : t))])

count :: [Int] -> Int -> Int
count [_h] num = num
count (h : t) num = count t (num + fromEnum (head t > h))

part1 :: String -> Int
part1 input = count (readInputFile input) 0

part2 :: String -> Int
part2 input = count (average (readInputFile input) []) 0

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 7
      part2 input `shouldBe` 5
