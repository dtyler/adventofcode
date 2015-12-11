require 'infinite_grid_walker'

RSpec.describe InfiniteGridWalker do
  it 'computes deliveries' do
    expect(InfiniteGridWalker.new('>').compute).to eq 2
    expect(InfiniteGridWalker.new('^>v<').compute).to eq 4
    expect(InfiniteGridWalker.new('^v^v^v^v^v').compute).to eq 2
  end
end
