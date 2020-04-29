function Initialize(loader)
end

function Run(queue)
  local range = UVR.create_range(0, 0, 1, 1);
  local lattice = UVR.create_lattice(10, 10);
  for y = 1, 11 do
    for x = 1, 11 do
      lattice[y][x][1] = math.sin((x - 1) / 10.0 * 2.0 * math.pi);
      lattice[y][x][2] = math.sin((y - 1) / 10.0 * 2.0 * math.pi);
    end
  end

  queue:push("default", range, nil, lattice);
  return queue;
end
