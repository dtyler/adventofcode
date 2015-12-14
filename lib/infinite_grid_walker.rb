require 'set'
# day3
class InfiniteGridWalker
  def initialize(input)
    @home = {}
    @home['0,0'] = 1
    @input = input
  end

  def compute
    x = y = 0
    @input.each_char do |symbol|
      case symbol
      when '^'
        x += 1
      when '>'
        y += 1
      when 'v'
        x -= 1
      when '<'
        y -= 1
      end
      update_home(x, y)
    end
    puts "\n#{@home}"
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
