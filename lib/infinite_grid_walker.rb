class InfiniteGridWalker
  def initialize(input)
    @home = { Point.new(0,0) => 1 }
    @input = input
  end

  def compute
    @home.size
  end

  class Point
    def initialize(x, y)
      @x, @y = x,y
    end

    attr_reader :x, :y
  end
end
