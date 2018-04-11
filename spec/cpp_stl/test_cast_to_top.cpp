// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include <cast_to_top.h>
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_cast_to_top) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    cast_to_top_t* r = new cast_to_top_t(&ks);

    BOOST_CHECK_EQUAL(r->code(), 80);
    BOOST_CHECK_EQUAL(r->header()->code(), 65);
    BOOST_CHECK_EQUAL(r->header_casted()->code(), 65);

    delete r;
}
