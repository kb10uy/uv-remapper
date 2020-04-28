Lattice = {}

for y = 1, 11 do
  Lattice[y] = {}
  for x = 1, 11 do
    local xsin = math.sin((x - 1) / 10.0 * 2.0 * math.pi);
    local ysin = math.sin((y - 1) / 10.0 * 2.0 * math.pi);
    Lattice[y][x] = {xsin, ysin};
  end
end
