---
name: Challenger submission template
about: Template for challenger submissions
title: "[Challenger-submission]"
labels: challenger
assignees: ''

---

```lua
-- IMPORTANT: By submitting this code you agree that your 
-- github username will be saved and shown on this profile

-- All Lua code within this code block will be executed
-- Code outside the onTurn function will only run once
-- If you want any persistent state between turns it might 
-- therefore be good to declare it here

-- This program will just go upwards

function onTurn(context)
    -- This code will run on every turn

    -- context has the following structure
    -- { 
    --    player={x, y, wallCount}, 
    --    opponent={x, y, wallCount}, 
    --    board=[0,0,0,0....] 9x9 long (one-dimensional) list containing tiles
    -- }
    -- Valid tiles are: { 0: Empty, 1: Player_One, 2: Player_two, 3: Wall} 

    -- Valid return types are UP=0, RIGHT=1, DOWN=2, LEFT=3 and wall
    -- Example if we want to place a wall from (1,1) to (1,2) then we would
    -- return "1,1,1,2"
    return "0"
end

function onJump(context)
    -- Valid returns are 0, 1, 2, 3, for more details read the docs
    return "0"
end

```