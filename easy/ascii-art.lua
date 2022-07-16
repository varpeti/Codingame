local X = tonumber(io.read())
local Y = tonumber(io.read())
local text = io.read()
io.stderr:write(X.." "..Y.." "..text.."\n")

local ch =
{
    ["A"]=0,
    ["B"]=1,
    ["C"]=2,
    ["D"]=3,
    ["E"]=4,
    ["F"]=5,
    ["G"]=6,
    ["H"]=7,
    ["I"]=8,
    ["J"]=9,
    ["K"]=10,
    ["L"]=11,
    ["M"]=12,
    ["N"]=13,
    ["O"]=14,
    ["P"]=15,
    ["Q"]=16,
    ["R"]=17,
    ["S"]=18,
    ["T"]=19,
    ["U"]=20,
    ["V"]=21,
    ["W"]=22,
    ["X"]=23,
    ["Y"]=24,
    ["Z"]=25,
    ["?"]=26
}

local IN  = {}
local OUT = {}

for y=1,Y do
    local row = io.read()
    IN[y]  = row;
    OUT[y] = "";
end

for i=1, #text do
    local letter = text:sub(i,i):upper()
    
    for y=1,Y do
        local row = IN[y]
        
        local x = ch[letter]
        if not x then x=26 end
        local out = row:sub(x*X+1,x*X+X) --i-től j-ig j benne van
        
        OUT[y]=OUT[y]..out
    end
end

for y=1,Y do
    print(OUT[y])
end



