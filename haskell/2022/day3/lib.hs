import Test.Hspec
import Data.List
import System.Environment (getArgs)

checkRuck :: [String] -> Int
checkRuck = sum . map (calValue . findChar . splitHalf)
  where
    splitHalf str = [take (length str `div` 2) str, drop (length str `div` 2) str]

checkRucks :: [String] -> Int
checkRucks [] = 0
checkRucks (x : y : z : rest) = calValue (findChar [x, y, z]) + checkRucks rest

calValue :: [Char] -> Int
calValue = sum . map (\c -> maybe 0 (+ 1) $ elemIndex c alphabet)

findChar :: [String] -> [Char]
findChar [a, b] = nub (filter (`elem` b) a)
findChar [a, b, c] = nub (filter (\ch -> ch `elem` b && ch `elem` c) a)
findChar _ = []

alphabet :: String
alphabet = ['a' .. 'z'] ++ ['A' .. 'Z']

part1 :: String -> Int
part1 file = checkRuck $ lines file

part2 :: String -> Int
part2 file = checkRucks $ lines file

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 157
      part2 input `shouldBe` 70
