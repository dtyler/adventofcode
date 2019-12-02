total_area = 0
ARGF.each do |line|
  length, width, height = line.split('x')
  l = length.to_i
  w = width.to_i
  h = height.to_i
  face1 = 2 * l * w
  face2 = 2 * w * h
  face3 = 2 * h * l
  scrap = [face1, face2, face3].min / 2
  area = face1 + face2 + face3
  puts area, scrap
  total_area = total_area + area + scrap
end
puts total_area
