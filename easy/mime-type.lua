-- Auto-generated code below aims at helping you parse
-- the standard input according to the problem statement.

local N = tonumber(io.read()) -- Number of elements which make up the association table.
local Q = tonumber(io.read()) -- Number Q of file names to be analyzed.

local mimes = {}

for i=0,N-1 do
    -- EXT: file extension
    -- MT: MIME type.
    local next_token = string.gmatch(io.read(), "[^%s]+")
    local ext = next_token():upper()
    local mt = next_token()
    mimes[ext] = mt
end

local ret = ""

for i=0,Q-1 do
    local token = string.gmatch(io.read(), ".[^.]*")
    local last = nil
    for w in token do
        last = w:sub(2):upper()
    end
    if mimes[last] then
        ret = ret..mimes[last].."\n"
    else 
        ret = ret.."UNKNOWN\n" 
    end
end

-- Write an action using print()
-- To debug: io.stderr:write("Debug message\n")


-- For each of the Q filenames, display on a line the corresponding MIME type. If there is no corresponding type, then display UNKNOWN.
print(ret)
