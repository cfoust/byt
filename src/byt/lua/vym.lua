-- NOTE: This is just me trying to figure out what the API should be
-- for mutator programming in Lua. This may work eventually, but
-- right now it's just playing around.

-- TODO to get this working:
--

-- Make a new file mutator, which only works in file contexts.
local vym = Mutator:file("vym");
-- Alternatively: Mutator:editor("some_global_mutator");

-- Make new binding tables for each mode.
local normal_binds = vym:bind_table();
local insert_binds = vym:bind_table();
vym:set_bind_table(normal_binds);

-- An enum of all of the states.
local NORMAL, INSERT = 0, 1;
local MODE = NORMAL;

local function to_normal_mode()
  MODE = NORMAL;
  vym:set_bind_table(normal_binds);
end

local function to_insert_mode()
  MODE = INSERT;
  vym:set_bind_table(insert_binds);
end

-- the BindingTable:bind() function can take the following:
--    -- (sequence : String, action : String) Bind a singular action to a sequence.
--    -- (sequence : String, table : BindingTable) Move to this table when this sequence is entered.
--    -- (table : {
--      "key sequence" => "action" or BindingTable
--    }) Perform all of these bindings.
--
-- Examples:
-- table:bind("a", "delete_everything"); -- Bind the a key to delete_everything
-- table:bind("a", other_table); -- Moves to other_table
normal_binds:bind({
  k = "up",
  j = "down",
  l = "right",
  h = "left"
});
-- The above sets a bunch of bindings at once

-- Actions occur at certain state transitions in the table.
vym:action("up", function(file)
  file:move_cursor_up();
end);

vym:action("down", function(file)
  file:move_cursor_down();
end);

vym:action("right", function(file)
  file:move_cursor_right();
end);

vym:action("left", function(file)
  file:move_cursor_left();
end);
