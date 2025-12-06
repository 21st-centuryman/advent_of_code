import Test.Hspec
import Data.Char (isDigit)
import System.Environment (getArgs)
import Data.Char (isDigit)

calcDirValue :: [String] -> [String] -> Int -> [String] -> [Int]
calcDirValue _    []          val _   = [val]
calcDirValue list (x : xs)    val dir =
  case words x of
    "dir" : name : _ ->
      let newDir = dir ++ [name]
          find   = navTo list xs dir newDir
      in  case find of
            fHead : _ ->
              calcDirValue list xs (val + fHead) dir ++ find
            [] ->
              calcDirValue list xs val dir
    w : _ | not (null w) && isDigit (w !! 0) ->
      calcDirValue list xs (val + read w) dir
    cmd | length cmd == 3 ->
      [val]
    _ ->
      calcDirValue list xs val dir

navTo :: [String] -> [String] -> [String] -> [String] -> [Int]
navTo list [] current goal = navTo list list ["/"] goal
navTo list (x : xs) current goal
  | current == goal = calcDirValue list xs 0 current
  | length command == 3 && last command == ".." = navTo list xs (init current) goal
  | length command == 3 && command !! 1 == "cd" = navTo list xs (current ++ [last command]) goal
  | otherwise = navTo list xs current goal
  where
    command = words x

createList :: [String] -> [Int]
createList (x : xs) = calcDirValue xs xs 0 ["/"]

part1 :: String -> Int
part1 file = sum $ filter (<= 100000) $ createList $ lines file

part2 :: String -> Int
part2 file =
  let xs@(total : _) = createList (lines file)
      sizeToDel = 30000000 - (70000000 - total)
  in  minimum $ filter (>= sizeToDel) xs

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 95437
      part2 input `shouldBe` 24933642
