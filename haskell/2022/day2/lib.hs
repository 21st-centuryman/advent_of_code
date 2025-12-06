import Test.Hspec
import System.Environment (getArgs)

game :: [String] -> Int -> Int
game [] status = 0
game (h : t) status =  case words h of
    [x, y] ->
      case status of
        1 -> gameRound  [x, y] + game t 1
        2 -> gameRound2 [x, y] + game t 2
gameRound :: [String] -> Int
gameRound [] = 0
gameRound [x, y] =
  case y of
    "X" -> case x of "A" -> 4; "B" -> 1; "C" -> 7
    "Y" -> case x of "A" -> 8; "B" -> 5; "C" -> 2
    "Z" -> case x of "A" -> 3; "B" -> 9; "C" -> 6

gameRound2 :: [String] -> Int
gameRound2 [] = 0
gameRound2 [x, y] =
  case y of
    "X" -> case x of "A" -> 3; "B" -> 1; "C" -> 2
    "Y" -> case x of "A" -> 4; "B" -> 5; "C" -> 6
    "Z" -> case x of "A" -> 8; "B" -> 9; "C" -> 7

part1 :: String -> Int
part1 file = game (lines file) 1

part2 :: String -> Int
part2 file = game (lines file) 2

main :: IO ()
main = do
  args <- getArgs
  case args of
    [path] -> do
      input <- readFile path
      part1 input `shouldBe` 15
      part2 input `shouldBe` 12
