module Day1.Lib where

import Data.List (sort)
import Data.List.Split (splitOn)

part1 :: String -> Int
part1 = maximum . map (sum . map read . lines) . splitOn "\n\n"

part2 :: String -> Int
part2 = sum . take 3 . reverse . sort . map (sum . map read . lines) . splitOn "\n\n"
