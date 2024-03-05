#include "pch.h"

using namespace utils;

constexpr auto DEBUG_COLOR = 0x0B;

Logger::Level Logger::GetLevelData(const Levels level)
{
    switch (level)
    {
    case Levels::Trace:
        return { 0x08, "Trace" };
    case Levels::Debug:
        return { DEBUG_COLOR, "Debug" };
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
    // If the console is disabled, don't log anything.
    if (!snowflake::show_console) return;

    std::cout << "[Snowflake] " << message << std::endl;
}

void Logger::log(const Level level, const std::string& message)
{
    // If the console is disabled, don't log anything.
    if (!snowflake::show_console) return;
    // If debug logging is disabled, don't log debug messages.
    if (!snowflake::log_debug && level.color == DEBUG_COLOR) return;

    std::cout << "[";

    const auto hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, level.color);
    std::cout << level.text;
    SetConsoleTextAttribute(hConsole, 15);

    std::cout << "] " << message << std::endl;
}
