function Initialize(loader)
  loader:load("natsuki", "images/natsuki.png");
  return loader;
end

function Run(queue)
  local range = UVR.create_range(0, 0, 1, 1);
  local lattice = UVR.create_lattice(1000, 1000);
  for y = 1, 1001 do
    for x = 1, 1001 do
      lattice[y][x][1] = math.sin((x - 1) / 1000.0 * 2.0 * math.pi) + 0.5 * math.sin(3.0 * (y - 1) / 1000.0 * 2.0 * math.pi);
      lattice[y][x][2] = math.sin((y - 1) / 1000.0 * 2.0 * math.pi);
    end
  end

  -- queue:push("default_uv", range, nil, lattice);
  queue:push("natsuki", range, nil, lattice);
  return queue;
end
