function Initialize(loader)
  loader:load("default_mask", "default_mask.png");
end

function Run(remapper)
  remapper:push()
end

function Patch(queue)
  local range = uvr:new_range(0, 0, 128, 128);
  local lattice = uvr:new_lattice(40, 40);
  queue:push("default", "default_mask", range, lattice);

  return uvr
end
