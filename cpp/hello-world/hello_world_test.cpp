#include "hello_world.h"
#define BOOST_TEST_MAIN
#include <boost/test/unit_test.hpp>

<<<<<<< HEAD
BOOST_AUTO_TEST_CASE(test_hello) {
=======
BOOST_AUTO_TEST_CASE(test_hello)
{
>>>>>>> feature/cpp
    BOOST_REQUIRE_EQUAL("Hello, World!", hello_world::hello());
}
