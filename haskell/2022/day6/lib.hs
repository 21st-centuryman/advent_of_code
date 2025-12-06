import Test.Hspec
import Data.List (nub)
import System.Environment (getArgs)

checkString :: String -> Int -> Int
checkString str val
  | length group == val = val
  | otherwise = 1 + checkString (drop 1 str) val
  where
    group = nub $ take val str

part1 :: String -> Int
part1 str = checkString str 4

part2 :: String -> Int
part2 str = checkString str 14

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 7
      part2 input `shouldBe` 19
