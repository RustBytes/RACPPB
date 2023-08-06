class IntegerWrapper {
public:
  IntegerWrapper();
  IntegerWrapper(int value);

  int Get();
  void Set(int value);

  IntegerWrapper(IntegerWrapper &&) = default;
  IntegerWrapper(const IntegerWrapper &) = default;
  IntegerWrapper &operator=(IntegerWrapper &&) = default;
  IntegerWrapper &operator=(const IntegerWrapper &) = default;

  ~IntegerWrapper();

private:
  int inner;
};
