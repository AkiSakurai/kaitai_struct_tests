// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include <str_encodings.h>
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_str_encodings) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_encodings_t* r = new str_encodings_t(&ks);

    BOOST_CHECK_EQUAL(r->str1(), std::string("Some ASCII"));
    BOOST_CHECK_EQUAL(r->str2(), std::string("\u3053\u3093\u306b\u3061\u306f"));
    BOOST_CHECK_EQUAL(r->str3(), std::string("\u3053\u3093\u306b\u3061\u306f"));
    BOOST_CHECK_EQUAL(r->str4(), std::string("\u2591\u2592\u2593"));

    delete r;
}
