#pragma once

class String
{
public:
    void* klass;
    void* monitor;
    uint32_t length;
    wchar_t chars[];

    wchar_t* c_str() {
        return chars;
    }

    size_t size() {
        return length;
    }
};
