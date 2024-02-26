#pragma once

namespace utils
{
    void create_console();

    class Console
    {
    public:
        static std::unordered_map<std::string, void*> commands;

        static void ConsoleReader();
        static void CreateConsoleThread();

        static void ProcessCommand(const std::string& command);
    };
}
