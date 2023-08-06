#include "lib.hpp"

// Default constructor
IntegerWrapper::IntegerWrapper() {
  inner = 0;
}

// With value constructor
IntegerWrapper::IntegerWrapper(int value) {
  inner = value;
}

// Get inner value.
int IntegerWrapper::Get() {
  return inner;
}

// Set inner value
void IntegerWrapper::Set(int value) {
  inner = value;
}
