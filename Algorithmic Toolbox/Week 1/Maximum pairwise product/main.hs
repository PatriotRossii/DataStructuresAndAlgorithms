main = do
    line <- getLine
    let a = map read (words line) :: [Int]
        l = length b
        c = maximum [i * j | i <- [0..l], j <- [i + 1..l]]
        in print c