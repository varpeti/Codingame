local shape = {}

local N = tonumber(io.read())
for i=0,N-1 do
    local next_token = string.gmatch(io.read(), "[^%s]+")
    local x = tonumber(next_token())
    local y = tonumber(next_token())
    table.insert(shape,{x=x,y=y})
end

local points = {}

local M = tonumber(io.read())
for i=0,M-1 do
    local next_token = string.gmatch(io.read(), "[^%s]+")
    local x = tonumber(next_token())
    local y = tonumber(next_token())
    table.insert(points,{x=x,y=y})
end

-- Write an action using print()
-- To debug: io.stderr:write("Debug message\n")

function sign(p1, p2, p3)
    return (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
end

function isPointInTriangle(p, p0, p1, p2)
    local A = 1/2 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
    local sign = 1 if A<0 then sign = -1 end
    local s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * p.x + (p0.x - p2.x) * p.y) * sign;
    local t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * p.x + (p1.x - p0.x) * p.y) * sign;
    
    return s>=0 and t>=0 and (s+t)<=(2*A*sign);
end

function copyPoint(b)
    local a = {x=b.x,y=b.y}
    return a
end

function copyShape(b)
    local a = {}
    for i,point in ipairs(b) do
        table.insert(a, copyPoint(point))
    end
    return a
end

function shape2Triangles(shape)
    local triangles = {}

    local s = copyShape(shape)

    while #s>=3 do
        local triangle = {{x=0,y=0},{x=0,y=0},{x=0,y=0}}
        triangle[1] = copyPoint(s[1])
        triangle[2] = copyPoint(s[2])
        triangle[3] = copyPoint(s[3])
        table.remove(s,2)
        table.insert(triangles,triangle)
    end
    return triangles
end

function isInside(shape,point)
    local triangles = shape2Triangles(shape)

    for i,triangle in ipairs(triangles) do
        if isPointInTriangle(point,triangle[1],triangle[2],triangle[3]) then return true end
    end
    return false
end

for p,point in ipairs(points) do
    if isInside(shape,point) then
        print("hit")
    else
        print("miss")
    end
end
