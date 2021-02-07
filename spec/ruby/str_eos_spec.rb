# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'str_eos'
rescue SyntaxError => e
  load_err = e
  StrEos = nil
rescue LoadError => e
  load_err = e
  StrEos = nil
end

RSpec.describe StrEos do
  it 'parses test properly' do
    raise load_err if StrEos.nil?
    r = StrEos.from_file('src/term_strz.bin')

    expect(r.str).to eq "foo|bar|baz@"
  end
end
