--[[
	This is a breath first search implementation
]]
MAP_SIZE = 9

-- By specifying a current tile and a previous adjacent tile
-- this function returns should move to get back to the previous tile
local function getDirection(fromX, fromY, toX, toY)
	if fromX == toX then
		-- Down
		if fromY == toY + 1 then
			return 2
		else -- Up
			return 0
		end
	else
		-- Right
		if fromX == toX + 1 then
			return 1
		else -- Left
			return 3
		end
	end
end

local function coordinateFromDirection(x, y, direction)
	if direction == 0 then
		return { x = x, y = y - 1 }
	elseif direction == 1 then
		return { x = x + 1, y = y }
	elseif direction == 2 then
		return { x = x, y = y + 1 }
	else
		return { x = x - 1, y = y }
	end
end

local function getIndex(coordinate)
	return coordinate.y * MAP_SIZE + coordinate.x + 1
end

local function bfs(x, y, context)
	local queue = { { x = x, y = y } }
	local lastMoveQueue = { { x = x, y = y } }
	local prevTile = { x = x, y = y }
	local goal = nil
	-- Construct empty visisted map
	local visited = {}
	for i = 1, MAP_SIZE * MAP_SIZE, 1 do
		table.insert(visited, -1)
	end

	local counter = 0

	-- Continue searching until either all tiles have been visited
	-- or the top of the map has been reached
	while next(queue) ~= nil do
		counter = counter + 1
		local current = table.remove(queue, 1)
		local prevTile = table.remove(lastMoveQueue, 1)
		-- Here we mark how to get back to the previous tile
		-- from our current position
		visited[getIndex(current)] = getDirection(prevTile.x, prevTile.y, current.x, current.y)
		if current.y == 0 then
			goal = { x = current.x, y = current.y }
			break
		end

		-- Check all adjacent tiles
		for i = 0, 3, 1 do
			-- Create a new table to avoid modifying the current coordinate
			local new_coordinate = { x = current.x, y = current.y }
			if i == 0 then
				new_coordinate.y = new_coordinate.y - 1
			elseif i == 1 then
				new_coordinate.x = new_coordinate.x + 1
			elseif i == 2 then
				new_coordinate.y = new_coordinate.y + 1
			else
				new_coordinate.x = new_coordinate.x - 1
			end

			-- Check if coordinate is within bounds
			if new_coordinate.x >= 0 and new_coordinate.x < MAP_SIZE and new_coordinate.y >= 0 and new_coordinate.y < MAP_SIZE then
				local tile = context.board[getIndex(new_coordinate)]

				-- Pathing algorithm is allowed to path through empty tiles, player 1 and player 2
				if tile < 3 and visited[getIndex(new_coordinate)] == -1 then
					--[[ return "#debug inside " .. tostring(new_coordinate.x) .. " " .. tostring(new_coordinate.y) .. "\n" ]]
					table.insert(lastMoveQueue, current)
					table.insert(queue, new_coordinate)
				end
			end
		end
	end

	local current = { x = goal.x, y = goal.y }
	local prev = { x = goal.x, y = goal.y }
	while true do
		-- Continue until we have found our starting position
		if current.x == x and current.y == y then
			-- Invert direction
			return tostring((getDirection(current.x, current.y, prev.x, prev.y) + 2) % 4);
		end
		prev = { x = current.x, y = current.y }

		if current.x < 0 or current.x >= MAP_SIZE or current.y < 0 or current.y >= MAP_SIZE then
			return "#debug 4 " .. tostring(current.x) .. " " .. tostring(current.y)
		end

		current = coordinateFromDirection(current.x, current.y, visited[getIndex(current)])
	end
end


function onTurn(context)
	return bfs(context.player.x, context.player.y, context)
end

function onJump(context)
	return "0"
end
