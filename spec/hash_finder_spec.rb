require 'hash_finder'

RSpec.describe HashFinder do
  it 'finds decimal pad for secret key' do
    expect(HashFinder.new('abcdef').compute).to eq 609_043
    expect(HashFinder.new('pqrstuv').compute).to eq 1_048_970
  end
end
