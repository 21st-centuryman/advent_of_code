module Day3.Lib where

import Data.List

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
findChar strings
  | length strings == 2 = nub (filter (`elem` last strings) $ head strings)
  | length strings == 3 = nub (filter (\c -> c `elem` (strings !! 1) && c `elem` last strings) $ head strings)
  | otherwise = []

alphabet :: String
alphabet = ['a' .. 'z'] ++ ['A' .. 'Z']

part1 :: String -> Int
part1 file = checkRuck $ lines file

part2 :: String -> Int
part2 file = checkRucks $ lines file
