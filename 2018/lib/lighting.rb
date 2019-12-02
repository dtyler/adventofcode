class Lighting
  def initialize(input)
    @input = input
    @grid = Array.new(1_000_000) {|i| Array.new(1_000_000, 0) }
  end

  def compute
    @grid
  end
end
