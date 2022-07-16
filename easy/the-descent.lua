

while true do
    local mountain = {}
    for i=1,8 do
         table.insert(mountain,tonumber(io.read())) 
    end
    -- To debug: io.stderr:write("Debug message\n")
    local max = {id=1,h=mountain[1]}
    for id,h in ipairs(mountain) do
        if max.h<h then 
            max.id=id
            max.h=h
        end
    end
    
    print(max.id-1) -- The index of the mountain to fire on.
end
