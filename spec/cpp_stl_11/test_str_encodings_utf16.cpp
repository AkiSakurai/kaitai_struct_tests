// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "str_encodings_utf16.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_str_encodings_utf16) {
    std::ifstream ifs("src/str_encodings_utf16.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_encodings_utf16_t* r = new str_encodings_utf16_t(&ks);

    BOOST_CHECK_EQUAL(r->len_be(), 12);
    BOOST_CHECK_EQUAL(r->be_bom_removed()->bom(), 65279);
    BOOST_CHECK_EQUAL(r->be_bom_removed()->str(), std::string("\u3053\u3093\u306b\u3061\u306f"));
    BOOST_CHECK_EQUAL(r->len_le(), 12);
    BOOST_CHECK_EQUAL(r->le_bom_removed()->bom(), 65279);
    BOOST_CHECK_EQUAL(r->le_bom_removed()->str(), std::string("\u3053\u3093\u306b\u3061\u306f"));

    delete r;
}
