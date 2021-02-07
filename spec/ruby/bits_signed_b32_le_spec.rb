# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'bits_signed_b32_le'
rescue SyntaxError => e
  load_err = e
  BitsSignedB32Le = nil
rescue LoadError => e
  load_err = e
  BitsSignedB32Le = nil
end

RSpec.describe BitsSignedB32Le do
  it 'parses test properly' do
    raise load_err if BitsSignedB32Le.nil?
    r = BitsSignedB32Le.from_file('src/bits_signed_b32_le.bin')

    expect(r.a_num).to eq 0
    expect(r.a_bit).to eq true
    expect(r.b_num).to eq 2147483647
    expect(r.b_bit).to eq false
  end
end
