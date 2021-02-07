# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'integers'
rescue SyntaxError => e
  load_err = e
  Integers = nil
rescue LoadError => e
  load_err = e
  Integers = nil
end

RSpec.describe Integers do
  it 'parses test properly' do
    raise load_err if Integers.nil?
    r = Integers.from_file('src/fixed_struct.bin')

    expect(r.uint8).to eq 255
    expect(r.uint16).to eq 65535
    expect(r.uint32).to eq 4294967295
    expect(r.uint64).to eq 18446744073709551615
    expect(r.sint8).to eq -1
    expect(r.sint16).to eq -1
    expect(r.sint32).to eq -1
    expect(r.sint64).to eq -1
    expect(r.uint16le).to eq 66
    expect(r.uint32le).to eq 66
    expect(r.uint64le).to eq 66
    expect(r.sint16le).to eq -66
    expect(r.sint32le).to eq -66
    expect(r.sint64le).to eq -66
    expect(r.uint16be).to eq 66
    expect(r.uint32be).to eq 66
    expect(r.uint64be).to eq 66
    expect(r.sint16be).to eq -66
    expect(r.sint32be).to eq -66
    expect(r.sint64be).to eq -66
  end
end
