main = do
    line <- getLine
    let a = words line
        b = map read a
        in print $ sum b