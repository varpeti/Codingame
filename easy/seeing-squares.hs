import System.IO
import Control.Monad
import Data.Array

dbg :: Show a => String -> a -> IO ()
dbg name value = hPutStrLn stderr (name ++ " = " ++ show value)

main :: IO ()
main = do
    hSetBuffering stdout NoBuffering -- DO NOT REMOVE

    [r, c] <- map read . words <$> getLine
    lines <- replicateM r getLine

    let grid = listArray ((0,0), (r-1, c-1)) [lines!!i!!j | i <- [0..r-1], j <- [0..c-1]]

    let boxes = [ ()
                | x1 <- [0..r-1]
                , y1 <- [0..c-1]
                , grid ! (x1, y1) == '+'
                , x2 <- [x1+1..r-1]
                , y2 <- [y1+1..c-1]
                , y2 - y1 == 2 * (x2 - x1)
                , isBox grid x1 y1 x2 y2
                ]

    putStrLn (show (length boxes))
    return ()

isBox :: Array (Int, Int) Char -> Int -> Int -> Int -> Int -> Bool
isBox grid x1 y1 x2 y2 =
    grid ! (x2, y2) == '+' &&
    grid ! (x1, y2) == '+' &&
    grid ! (x2, y1) == '+' &&
    validHorizontal grid x1 y1 y2 &&
    validHorizontal grid x2 y1 y2 &&
    validVertical grid y1 x1 x2 &&
    validVertical grid y2 x1 x2

validHorizontal :: Array (Int, Int) Char -> Int -> Int -> Int -> Bool
validHorizontal grid x y1 y2 = all (`elem` "-+") [grid!(x, y) | y <- [y1..y2]]

validVertical :: Array (Int, Int) Char -> Int -> Int -> Int -> Bool
validVertical grid y x1 x2 = all (`elem` "|+") [grid!(x, y) | x <- [x1..x2]
