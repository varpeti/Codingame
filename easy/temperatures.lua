-- Auto-generated code below aims at helping you parse
-- the standard input according to the problem statement.

local n = tonumber(io.read()) -- the number of temperatures to analyse
local next_token = string.gmatch(io.read(), "[^%s]+")
local min = 5527
for i=1,n do
    -- t: a temperature expressed as an integer ranging from -273 to 5526
    local t = tonumber(next_token())
    if math.abs(t)<=math.abs(min) then
        if math.abs(t)==math.abs(min) and t~=min then 
            min=math.abs(t)
        else 
            min=t
        end
    end
end

-- Write an action using print()
-- To debug: io.stderr:write("Debug message\n")
if min==5527 then min=0 end
print(min)
