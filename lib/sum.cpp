#include <iostream>

// 必须用 extern "C" 禁止 C++ 名称修饰（Name Mangling）
extern "C" {
    int add(int a, int b) {
        return a + b;
    }
}
