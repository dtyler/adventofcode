require 'ribbon_calc'

RSpec.describe RibbonCalc do
  subject(:ribbon_calc) { RibbonCalc.new('') }
  it 'computes amount of ribbon needed' do
    expect(ribbon_calc.calculate(2, 3, 4)).to eq 34
    expect(ribbon_calc.calculate(1, 1, 10)).to eq 14
  end
end
