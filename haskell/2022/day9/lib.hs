import Test.Hspec
import Data.List (nub)
import System.Environment (getArgs)

part1 :: String -> Int
part1 = length . nub . ropeLength 1 . headPath [[0, 0]] . lines

-- part2 = length . nub . ropeLength 10 . headPath [[0, 0]] . lines

ropeLength :: Int -> [[Int]] -> [[Int]]
ropeLength 0 lst = lst
ropeLength n (x : xs) = ropeLength (n - 1) $ x : tailPath (x : xs)

readInstruction :: String -> [[Int]] -> [[Int]]
readInstruction str lst = case words str of
  ["R", num] -> lst ++ [[t0, t1 + x] | x <- [1 .. read num]]
  ["L", num] -> lst ++ [[t0, t1 - x] | x <- [1 .. read num]]
  ["U", num] -> lst ++ [[t0 + x, t1] | x <- [1 .. read num]]
  ["D", num] -> lst ++ [[t0 - x, t1] | x <- [1 .. read num]]
  _ -> []
  where
    [t0, t1] = last lst

headPath :: [[Int]] -> [String] -> [[Int]]
headPath = foldl $ flip readInstruction

tailPath :: [[Int]] -> [[Int]]
tailPath [] = []
tailPath (t : h : u : xs)
  | isNext    = tailPath (t : u : xs)
  | otherwise = h : tailPath (h : u : xs)
  where
    isNext = isNeightbor t u
tailPath _ = []

isNeightbor :: [Int] -> [Int] -> Bool
isNeightbor [t0, t1] [h0, h1] = (h0 - t0) ^ 2 + (h1 - t1) ^ 2 < 4

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 13

