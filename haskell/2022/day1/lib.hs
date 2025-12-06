import Data.List (sort)
import Data.List.Split (splitOn)
import Test.Hspec
import System.Environment (getArgs)

part1 :: String -> Int
part1 = maximum . map (sum . map read . lines) . splitOn "\n\n"

part2 :: String -> Int
part2 = sum . take 3 . reverse . sort . map (sum . map read . lines) . splitOn "\n\n"

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 24000
      part2 input `shouldBe` 45000
