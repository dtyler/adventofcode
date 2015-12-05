# Computes amount of ribbon needed
class RibbonCalc
  def initialize(input)
    total_ribbon = 0
    input.each_line do |line|
      sides = line.split('x').map(&:to_i)
      total_ribbon += compute(*sides)
    end
    puts total_ribbon
  end

  def compute(l, w, h)
    min_side1, min_side2 = [l, w, h].min(2)
    min_parameter = min_side1 * 2 + min_side2 * 2
    volume = l * w * h
    min_parameter + volume
  end
end
