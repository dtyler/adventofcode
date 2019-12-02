require 'infinite_grid_walker'

RSpec.describe InfiniteGridWalker do
  it 'computes deliveries' do
    expect(InfiniteGridWalker.new('>', 1).compute).to eq 2
    expect(InfiniteGridWalker.new('^>v<', 1).compute).to eq 4
    expect(InfiniteGridWalker.new('^v^v^v^v^v', 1).compute).to eq 2
  end

  it 'computes deliveries with multiple walkers' do
    expect(InfiniteGridWalker.new('^v', 2).compute).to eq 3
    expect(InfiniteGridWalker.new('^>v<', 2).compute).to eq 3
    expect(InfiniteGridWalker.new('^v^v^v^v^v', 2).compute).to eq 11
  end
end
