#pragma once

#define LOG_INFO(fmt, ...) utils::Logger::log(utils::Logger::GetLevelData(utils::Logger::Levels::Info), fmt, __VA_ARGS__)
#define LOG_DEBUG(fmt, ...) utils::Logger::log(utils::Logger::GetLevelData(utils::Logger::Levels::Debug), fmt, __VA_ARGS__)
#define LOG_TRACE(fmt, ...) utils::Logger::log(utils::Logger::GetLevelData(utils::Logger::Levels::Trace), fmt, __VA_ARGS__)
#define LOG_WARN(fmt, ...) utils::Logger::log(utils::Logger::GetLevelData(utils::Logger::Levels::Warning), fmt, __VA_ARGS__)
#define LOG_ERROR(fmt, ...) utils::Logger::log(utils::Logger::GetLevelData(utils::Logger::Levels::Error), fmt, __VA_ARGS__)

namespace utils
{
    class Logger
    {
    public:
        enum class Levels
        {
            Trace,
            Debug,
            Info,
            Warning,
            Error
        };

        struct Level
        {
            char color;
            const char* text;
        };

        static Level GetLevelData(Levels level);

        static void print(const std::string& message);
        static void log(Level level, const std::string& message);
    };
}
