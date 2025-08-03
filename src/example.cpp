// C++ 类 (example.cpp)
class Calculator {
public:
    int add(int a, int b) { return a + b; }
};

extern "C" {
    Calculator* calculator_new() { return new Calculator(); }
    int calculator_add(Calculator* calc, int a, int b) { return calc->add(a, b); }
    void calculator_delete(Calculator* calc) { delete calc; }
}