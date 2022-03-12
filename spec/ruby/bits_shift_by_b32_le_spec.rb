# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'bits_shift_by_b32_le'
rescue SyntaxError => e
  load_err = e
  BitsShiftByB32Le = nil
rescue LoadError => e
  load_err = e
  BitsShiftByB32Le = nil
end

RSpec.describe BitsShiftByB32Le do
  it 'parses test properly' do
    raise load_err if BitsShiftByB32Le.nil?
    r = BitsShiftByB32Le.from_file('src/bits_shift_by_b32_le.bin')

    expect(r.a).to eq 4294967295
    expect(r.b).to eq 0
  end
end
