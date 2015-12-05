floor = 0
ARGF.each_char do |cur|
  floor += 1 if cur == '('
  floor -= 1 if cur == ')'
end
puts floor
