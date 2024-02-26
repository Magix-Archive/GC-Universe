#include "pch.h"
#include "Console.h"

#include "Utilities.hpp"

using namespace utils;

std::unordered_map<std::string, void*> Console::commands;

void utils::create_console()
{
    AllocConsole();
    AttachConsole(GetCurrentProcessId());

    freopen_s((FILE**)stdin, "CONIN$", "r", stdin);
    freopen_s((FILE**)stdout, "CONOUT$", "w", stdout);
    freopen_s((FILE**)stderr, "CONOUT$", "w", stderr);

    SetConsoleOutputCP(CP_UTF8);

    SetConsoleMode(stdout,
        ENABLE_PROCESSED_OUTPUT | ENABLE_WRAP_AT_EOL_OUTPUT);

    SetConsoleMode(stdin,
        ENABLE_INSERT_MODE | ENABLE_EXTENDED_FLAGS |
        ENABLE_PROCESSED_INPUT | ENABLE_QUICK_EDIT_MODE);

    Console::CreateConsoleThread();
}

void Console::ConsoleReader()
{
    for (std::string line; std::getline(std::cin, line);) {
        ProcessCommand(line);
    }
}

void Console::CreateConsoleThread()
{
    // TODO: Register commands.

    std::thread([]
    {
        ConsoleReader();
    }).detach();
}

void Console::ProcessCommand(const std::string& command)
{
    const auto split = utils::split(command, " ");
    if (split.size() == 0)
    {
        LOG_INFO("Provide a command to run. Run 'help' for a list of commands.");
        return;
    }

    if (const auto handler = commands.find(split[0].c_str());
        handler != commands.end())
    {
        ((void(*)())handler->second)();
    }
    else
    {
        LOG_INFO("Unknown command. Run 'help' for a list of commands.");
    }
}
