require 'repeat_n_strz_double'

RSpec.describe RepeatNStrzDouble do
  it 'parses test properly' do
    r = RepeatNStrzDouble.from_file('src/repeat_n_strz.bin')

    expect(r.qty).to eq 2
    expect(r.lines1).to eq ['foo']
    expect(r.lines2).to eq ['bar']
  end
end
