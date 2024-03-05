#pragma once

namespace utils
{
    inline std::vector<std::string> split(
        std::string str,
        const std::string& delimiter)
    {
        size_t cursorPosition = 0, cursorEnd;
        const size_t delimiterLength = delimiter.length();
        std::vector<std::string> result;

        while ((cursorEnd = str.find(delimiter, cursorPosition)) != std::string::npos) {
            std::string token = str.substr(cursorPosition, cursorEnd - cursorPosition);
            cursorPosition = cursorEnd + delimiterLength;
            result.push_back(token);
        }

        result.push_back(str.substr (cursorPosition));
        return result;
    }

    inline uintptr_t pattern_scan(LPCSTR module, LPCSTR pattern)
    {
        static auto pattern_to_byte = [](const char* pattern)
        {
            auto bytes = std::vector<int>{};
            auto start = const_cast<char*>(pattern);
            auto end = const_cast<char*>(pattern) + strlen(pattern);

            for (auto current = start; current < end; ++current)
            {
                if (*current == '?')
                {
                    ++current;
                    if (*current == '?')
                        ++current;
                    bytes.push_back(-1);
                }
                else
                {
                    bytes.push_back(strtoul(current, &current, 16));
                }
            }
            return bytes;
        };

        auto mod = GetModuleHandleA(module);
        if (!mod) return 0;

        auto dosHeader = (PIMAGE_DOS_HEADER) mod;
        auto ntHeaders = (PIMAGE_NT_HEADERS)((std::uint8_t*) mod + dosHeader->e_lfanew);
        auto sizeOfImage = ntHeaders->OptionalHeader.SizeOfImage;
        auto patternBytes = pattern_to_byte(pattern);
        auto scanBytes = reinterpret_cast<std::uint8_t*>(mod);
        auto s = patternBytes.size();
        auto d = patternBytes.data();

        for (auto i = 0ul; i < sizeOfImage - s; ++i)
        {
            bool found = true;
            for (auto j = 0ul; j < s; ++j)
            {
                if (scanBytes[i + j] != d[j] && d[j] != -1) {
                    found = false;
                    break;
                }
            }

            if (found)
            {
                return (uintptr_t) &scanBytes[i];
            }
        }

        return 0;
    }

    inline bool file_exists(const std::string& file_path)
    {
        return std::filesystem::exists(file_path);
    }
}
