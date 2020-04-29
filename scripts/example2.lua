-- 極座標変換
function Initialize(loader)
  return loader;
end

function Run(queue)
  local range = UVR.create_range(0, 0, 1, 1);
  local lattice = UVR.create_lattice(10, 10);
  for y = 1, 11 do
    local v = (y - 1) / 10.0 - 0.5;
    for x = 1, 11 do
      local u = (x - 1) / 10.0 - 0.5;
      local radius = math.sqrt(u ^ 2.0 + v ^ 2.0);
      local angle = math.atan(v, u);
      lattice[y][x][1] = radius * 2.0;
      lattice[y][x][2] = angle / (math.pi * 2.0);
    end
  end

  queue:push("default_uv", range, nil, lattice);
  return queue;
end
