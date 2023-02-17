--[[
	This bot is a dodger bot, it will always attemt
	to go to the finish line, if that isn't possible
	it will try to take another direction and hope
	hope for.

	If the opponent is one tile from winning the bot
	will try to place a wall in front of the opponent.
]]
-- START OF STATE --
local current_direction = 0 -- 0 UP, 1 RIGHT, 2 DOWN, 3 LEFT
--- END OF STATE ---

local function get_next_tile(x, y, direction)
	if direction == 0 then
		return {x, y - 1}
	elseif direction == 1 then
		return {x + 1, y}
	elseif direction == 2 then
		return {x, y + 1}
	elseif direction == 3 then
		return {x-1, y}
	end
	return {-1, -1}
end

local function turn_right()
	current_direction = (current_direction + 1) % 4
end
local function turn_left()
	current_direction = (current_direction + 3) % 4
end
local function place_wall(x1, y1, x2, y2)
	return tostring(x1) .. "," .. tostring(y1) .. "," .. tostring(x2) .. "," .. tostring(y2)
end

local function try_place_wall_in_front_of_player(context, player)
	-- If opponent is about to win we attempt to block that
	if STD__OCCUPIED(context, player.x, player.y + 1) == 0 and STD__OCCUPIED(context, player.x + 1, player.y + 1) then
		return place_wall(player.x, player.y + 1, player.x + 1, player.y + 1)
	elseif STD__OCCUPIED(context, player.x - 1, player.y + 1) == 0 and STD__OCCUPIED(context, player.x, player.y + 1) then
		return place_wall(player.x - 1, player.y + 1, player.x, player.y + 1)
	end
end

local function possible_to_turn_right(context, player, current_direction)
	local next_tile = get_next_tile(player.x, player.y, current_direction)
	return STD__OCCUPIED(context, next_tile[1], next_tile[2]) == 0
end

local function possible_to_turn_left(context, player, current_direction)
	local next_tile = get_next_tile(player.x, player.y, current_direction)
	return STD__OCCUPIED(context, next_tile[1], next_tile[2]) == 0
end

function onTurn(context)
	local player_x = context.player.x
	local player_y = context.player.y

	if context.opponent.y == 7 then
		try_place_wall_in_front_of_player(context, context.opponent)
	end

	-- If we are not walking up try to turn to face the finish line
	if (current_direction == 1 or current_direction == 2) and possible_to_turn_left(context, context.player, current_direction) then
		turn_left()
		return ""
	elseif (current_direction == 3) and possible_to_turn_right(context, context.player, current_direction) then
		turn_right()
		return ""
	end

	local next_coords = get_next_tile(player_x, player_y, current_direction)
	local is_occupied = STD__OCCUPIED(context, next_coords[1], next_coords[2])
	if not is_occupied then
		return tostring(current_direction)
	end
	turn_right()
	return onTurn(context)
end

function onJump(context)
	return "0"
end