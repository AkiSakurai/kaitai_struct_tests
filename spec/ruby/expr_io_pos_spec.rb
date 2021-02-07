# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'expr_io_pos'
rescue SyntaxError => e
  load_err = e
  ExprIoPos = nil
rescue LoadError => e
  load_err = e
  ExprIoPos = nil
end

RSpec.describe ExprIoPos do
  it 'parses test properly' do
    raise load_err if ExprIoPos.nil?
    r = ExprIoPos.from_file('src/expr_io_pos.bin')

    expect(r.substream1.my_str).to eq "CURIOSITY"
    expect(r.substream1.body).to eq [17, 34, 51, 68].pack('C*')
    expect(r.substream1.number).to eq 66
    expect(r.substream2.my_str).to eq "KILLED"
    expect(r.substream2.body).to eq [97, 32, 99, 97, 116].pack('C*')
    expect(r.substream2.number).to eq 103
  end
end
