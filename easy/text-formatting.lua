local s = io.read()

function nagyElso(s)
    return s:sub(1,1):upper() .. s:sub(2):lower()
end

function deleteLastSpace(s)
    return s:gsub("(.)[ ]+$", function(w) return w end)
end

function spaceAfter(s,p)
    return s:gsub("(["..p.."])", function(w) return w.." " end)
end

function onlyOne(s,p)
    return s:gsub("(["..p.."]+)", function(w) return p end)
end

function nagy(s)
    return s:gsub("[.][ ](.)", function(w) return ". "..w:upper() end)
end

function spaceBefore(s,p)
    return s:gsub("[ ]+(["..p.."])", function(w) return w end)
end

s = nagyElso(s)

s = spaceBefore(s,",")
s = spaceBefore(s,".")

s = onlyOne(s,",")
s = onlyOne(s,".")

s = spaceAfter(s,",")
s = spaceAfter(s,".")

s = onlyOne(s," ")
s = nagy(s)

s = deleteLastSpace(s)

print(s)
