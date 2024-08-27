// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "valid_fail_contents_inst.h"
#include <iostream>
#include <fstream>
#include <vector>
#include "kaitai/exceptions.h"

BOOST_AUTO_TEST_CASE(test_valid_fail_contents_inst) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    valid_fail_contents_inst_t* r = 0;
    BOOST_CHECK_THROW(
        r = new valid_fail_contents_inst_t(&ks),
        kaitai::validation_not_equal_error<std::string>
    );

    delete r;
}
