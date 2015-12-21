require 'lighting'


RSpec.describe Lighting do
  let(:all_on) do
   Array.new(1_000_000) {|i| Array.new(1_000_000, 1) } 
  end

  it 'responds to "turn on"' do
    expect(Lighting.new("turn on 0,0 through 999,999").compute).to eq all_on
  end
end
