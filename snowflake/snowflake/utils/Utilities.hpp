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
}
