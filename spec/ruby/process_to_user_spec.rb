# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'process_to_user'
rescue SyntaxError => e
  load_err = e
  ProcessToUser = nil
rescue LoadError => e
  load_err = e
  ProcessToUser = nil
end

RSpec.describe ProcessToUser do
  it 'parses test properly' do
    raise load_err if ProcessToUser.nil?
    r = ProcessToUser.from_file('src/process_rotate.bin')

    expect(r.buf1.str).to eq "Hello"
  end
end
