# Autogenerated from KST: please remove this line if doing any edits by hand!

require 'params_call_short'

RSpec.describe ParamsCallShort do
  it 'parses test properly' do
    r = ParamsCallShort.from_file('src/term_strz.bin')

    expect(r.buf1.body).to eq "foo|b"
    expect(r.buf2.body).to eq "ar|ba"
    expect(r.buf2.trailer).to eq 122
  end
end
