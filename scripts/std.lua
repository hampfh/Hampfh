-- Standard library file
-- Here we define all the standard functions that can be used by the bots

local MAP_SIZE = 9

function STD__CHECK_OUT_OF_BOUNDS(x, y)
	if x >= MAP_SIZE or y >= MAP_SIZE then
		return true
	end
	return false
end

function STD__OCCUPIED(gameObj, x, y)
	if STD__CHECK_OUT_OF_BOUNDS(x, y) then
		return true
	end

	if STD__GET_TILE(gameObj, x, y) ~= 0  then
		return true
	end

	return false
end


-- 0: Empty
-- 1: P1
-- 2: P2
-- 3: Wall
function STD__GET_TILE(gameObj, x, y)
	return gameObj.objects[x + MAP_SIZE * y]
end

