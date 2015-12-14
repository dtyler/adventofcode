require 'set'

# day3
class InfiniteGridWalker
  def initialize(input, walker_count)
    @home = {}
    @home['0,0'] = 1
    @input = input
    @walker_count = walker_count
  end

  def compute
    x = Array.new(@walker_count, 0)
    y = Array.new(@walker_count, 0)
    @input.each_char.with_index do |symbol, index|
      walker = index % @walker_count
      case symbol
      when '^'
        x[walker] += 1
      when '>'
        y[walker] += 1
      when 'v'
        x[walker] -= 1
      when '<'
        y[walker] -= 1
      end
      update_home(x[walker], y[walker])
    end
    @home.size
  end

  def update_home(x, y)
    if @home["#{x},#{y}"]
      @home["#{x},#{y}"] += 1
    else
      @home["#{x},#{y}"] = 1
    end
  end
end
