-- Auto-generated code below aims at helping you parse
-- the standard input according to the problem statement.
-- ---
-- Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.

-- lightX: the X position of the light of power
-- lightY: the Y position of the light of power
-- initialTX: Thor's starting X position
-- initialTY: Thor's starting Y position
next_token = string.gmatch(io.read(), "[^%s]+")
local l = {}
l.x = tonumber(next_token())
l.y = tonumber(next_token())
local t = {}
t.x = tonumber(next_token())
t.y = tonumber(next_token())

-- game loop
while true do
    local remainingTurns = tonumber(io.read()) -- The remaining amount of turns Thor can move. Do not remove this line.
    
    -- Write an action using print()
    -- To debug: io.stderr:write("Debug message\n")
    local dir = ""
    
    if (l.y>t.y) then 
        dir="S"
        t.y=t.y+1
    elseif (l.y<t.y) then 
        dir="N"
        t.y=t.y-1
    end
    
    if (l.x>t.x) then 
        dir=dir.."E"
         t.x=t.x+1
    elseif (l.x<t.x) then 
        dir=dir.."W"
        t.x=t.x-1
    end

    -- A single line providing the move to be made: N NE E SE S SW W or NW
    print(dir)
end
