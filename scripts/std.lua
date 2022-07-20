-- Standard library file
-- Here we define all the standard functions that can be used by the bots

local MAP_SIZE = 9

function STD__CHECK_OUT_OF_BOUNDS(x, y)
	return x >= MAP_SIZE or y >= MAP_SIZE
end

function STD__OCCUPIED(context, x, y)
	return STD__CHECK_OUT_OF_BOUNDS(x, y) or STD__GET_TILE(context, x, y) ~= 0
end


-- 0: Empty
-- 1: P1
-- 2: P2
-- 3: Wall
function STD__GET_TILE(context, x, y)
	-- Since lua always starts at 1 we need to compensate for that
	return context.board[x + MAP_SIZE * y + 1]
end

