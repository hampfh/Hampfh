-- Standard library file
-- Here we define all the standard functions that can be used by the bots

local MAP_SIZE = 9

function STD__CHECK_OUT_OF_BOUNDS(x, y)
	return x >= MAP_SIZE or y >= MAP_SIZE
end

function STD__OCCUPIED(context, x, y)
	return STD__CHECK_OUT_OF_BOUNDS(x, y) or STD__GET_TILE(context, x, y) ~= 0
end

function STD__PLAYER_OCCUPIED(context, x, y)
	return context.opponent.x == x and context.opponent.y == y
end

function STD__JUMP_POSSIBLE(context, direction)
	local x = context.player.x
	local y = context.player.y
	
	local jump_x = x
	local jump_y = y
	if direction == 0 then
		jump_y = y - 1
	elseif direction == 1 then
		jump_x = x + 1
	elseif direction == 2 then
		jump_y = y + 1
	elseif direction == 3 then
		jump_x = x - 1
	end

	-- If there is no player to jump over, then we can't jump
	if not STD__PLAYER_OCCUPIED(context, jump_x, jump_y) then
		return false
	end

	local jump_possible = false
	-- This is a special rule in the game, read more about this in the docs
	-- Lua bot documentation > The basics
	if jump_y - 1 ~= y and (not STD__OCCUPIED(context, jump_x, jump_y - 1) or jump_y == -1) then
		jump_possible = true
	elseif jump_x + 1 ~= x and not STD__OCCUPIED(context, jump_x + 1, jump_y) then
		jump_possible = true
	elseif jump_y + 1 ~= y and not STD__OCCUPIED(context, jump_x, jump_y + 1) then
		jump_possible = true
	elseif jump_x - 1 ~= x and not STD__OCCUPIED(context, jump_x - 1, jump_y) then
		jump_possible = true
	end
	return jump_possible
end

-- 0: Empty
-- 1: P1
-- 2: P2
-- 3: Wall
function STD__GET_TILE(context, x, y)
	-- Since lua always starts at 1 we need to compensate for that
	return context.board[x + MAP_SIZE * y + 1]
end

