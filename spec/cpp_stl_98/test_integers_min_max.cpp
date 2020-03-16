// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "integers_min_max.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_integers_min_max) {
    std::ifstream ifs("src/integers_min_max.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    integers_min_max_t* r = new integers_min_max_t(&ks);

    BOOST_CHECK_EQUAL(r->unsigned_min()->u1(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_min()->u2le(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_min()->u4le(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_min()->u8le(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_min()->u2be(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_min()->u4be(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_min()->u8be(), 0);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u1(), 255);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u2le(), 65535);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u4le(), 4294967295UL);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u8le(), 18446744073709551615ULL);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u2be(), 65535);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u4be(), 4294967295UL);
    BOOST_CHECK_EQUAL(r->unsigned_max()->u8be(), 18446744073709551615ULL);
    BOOST_CHECK_EQUAL(r->signed_min()->s1(), -128);
    BOOST_CHECK_EQUAL(r->signed_min()->s2le(), -32768);
    BOOST_CHECK_EQUAL(r->signed_min()->s4le(), -2147483648UL);
    BOOST_CHECK_EQUAL(r->signed_min()->s8le(), -9223372036854775808ULL);
    BOOST_CHECK_EQUAL(r->signed_min()->s2be(), -32768);
    BOOST_CHECK_EQUAL(r->signed_min()->s4be(), -2147483648UL);
    BOOST_CHECK_EQUAL(r->signed_min()->s8be(), -9223372036854775808ULL);
    BOOST_CHECK_EQUAL(r->signed_max()->s1(), 127);
    BOOST_CHECK_EQUAL(r->signed_max()->s2le(), 32767);
    BOOST_CHECK_EQUAL(r->signed_max()->s4le(), 2147483647);
    BOOST_CHECK_EQUAL(r->signed_max()->s8le(), 9223372036854775807LL);
    BOOST_CHECK_EQUAL(r->signed_max()->s2be(), 32767);
    BOOST_CHECK_EQUAL(r->signed_max()->s4be(), 2147483647);
    BOOST_CHECK_EQUAL(r->signed_max()->s8be(), 9223372036854775807LL);

    delete r;
}
