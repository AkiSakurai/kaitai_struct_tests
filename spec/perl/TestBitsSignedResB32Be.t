# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestBitsSignedResB32Be;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BitsSignedResB32Be;

sub test_bits_signed_res_b32_be: Test(1) {
    my $r = BitsSignedResB32Be->from_file('src/bits_shift_by_b32_le.bin');


    is($r->a(), 4294967295, 'Equals');
}

Test::Class->runtests;
