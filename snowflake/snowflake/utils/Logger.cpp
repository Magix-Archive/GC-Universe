#include "pch.h"

using namespace utils;

Logger::Level Logger::GetLevelData(const Levels level)
{
    switch (level)
    {
    case Levels::Trace:
        return { 0x08, "Trace" };
    case Levels::Debug:
        return { 0x0B, "Debug" };
    case Levels::Info:
        return { 0x02, "Info" };
    case Levels::Warning:
        return { 0x06, "Warning" };
    case Levels::Error:
        return { 0x0C, "Error" };
    }

    return {};
}


void Logger::print(const std::string& message)
{
    std::cout << "[Snowflake] " << message << std::endl;
}

void Logger::log(const Level level, const std::string& message)
{
    std::cout << "[";

    const auto hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, level.color);
    std::cout << level.text;
    SetConsoleTextAttribute(hConsole, 15);

    std::cout << "] " << message << std::endl;
}
