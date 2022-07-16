-- W: number of columns.
-- H: number of rows.
next_token = string.gmatch(io.read(), "[^%s]+")
W = tonumber(next_token())
H = tonumber(next_token())
local map = {}
for i=1,H do
    LINE = io.read().." " -- represents a line in the grid and contains W integers. Each integer represents one room of a given type.
    map[i] = {}
    for w in LINE:gmatch("%S+") do 
        table.insert(map[i],tonumber(w))
    end
end
EX = tonumber(io.read()) -- the coordinate along the X axis of the exit (not useful for this first mission, but must be read).

-- game loop
while true do
    next_token = string.gmatch(io.read(), "[^%s]+")
    XI = tonumber(next_token())+1
    YI = tonumber(next_token())+1
    POS = next_token()

    -- To debug: io.stderr:write("Debug message\n")
    
    local ret = {x=0,y=0}
    
    io.stderr:write(map[YI][XI])
    
    if map[YI][XI]==0 then
    elseif map[YI][XI]==1 then
        ret.x=XI
        ret.y=YI+1
    elseif map[YI][XI]==2 then
        if POS=="TOP" then
        elseif POS=="LEFT" then
            ret.x=XI+1
            ret.y=YI
        elseif POS=="RIGHT" then
            ret.x=XI-1
            ret.y=YI
        end
    elseif map[YI][XI]==3 then
        ret.x=XI
        ret.y=YI+1
    elseif map[YI][XI]==4 then
        if POS=="TOP" then
            ret.x=XI-1
            ret.y=YI
        elseif POS=="LEFT" then
        elseif POS=="RIGHT" then
            ret.x=XI
            ret.y=YI+1
        end
    elseif map[YI][XI]==5 then
        if POS=="TOP" then
            ret.x=XI+1
            ret.y=YI
        elseif POS=="LEFT" then
            ret.x=XI
            ret.y=YI+1
        elseif POS=="RIGHT" then
        end
    elseif map[YI][XI]==6 then
        if POS=="TOP" then
        elseif POS=="LEFT" then
            ret.x=XI+1
            ret.y=YI
        elseif POS=="RIGHT" then
            ret.x=XI-1
            ret.y=YI
        end
    elseif map[YI][XI]==7 then
        ret.x=XI
        ret.y=YI+1
    elseif map[YI][XI]==8 then
        ret.x=XI
        ret.y=YI+1
    elseif map[YI][XI]==9 then
        ret.x=XI
        ret.y=YI+1
    elseif map[YI][XI]==10 then
        ret.x=XI-1
        ret.y=YI
    elseif map[YI][XI]==11 then
        ret.x=XI+1
        ret.y=YI
    elseif map[YI][XI]==12 then
        ret.x=XI
        ret.y=YI+1
    elseif map[YI][XI]==13 then
        ret.x=XI
        ret.y=YI+1
    end

    -- One line containing the X Y coordinates of the room in which you believe Indy will be on the next turn.
    print((ret.x-1).." "..(ret.y-1) )
end
